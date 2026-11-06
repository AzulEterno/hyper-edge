//! Regression from hyperium/hyper#4233; preserve the response before handoff.
use bytes::Bytes;
use futures_util::future;
use http_body_util::Empty;
use hyper::{server::conn::http1, service::service_fn, Response};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[path = "../../tests/support/tokiort.rs"]
mod support;
use support::TokioIo;

#[tokio::test]
async fn upgrades_flush_buffered_response_before_handoff() {
    let prefix = b"HTTP/1.1";
    let (mut client, server) = tokio::io::duplex(prefix.len());

    let conn = http1::Builder::new().serve_connection(
        TokioIo::new(server),
        service_fn(|_| {
            let res = Response::builder()
                .status(101)
                .header("upgrade", "foobar")
                .body(Empty::<Bytes>::new())
                .unwrap();
            future::ready(Ok::<_, hyper::Error>(res))
        }),
    );
    let child = tokio::spawn(async move {
        let parts = conn.without_shutdown().await.unwrap();
        let mut io = parts.io.inner();
        io.write_all(b"foo=bar").await.unwrap();
        io.shutdown().await.unwrap();
    });

    client
        .write_all(
            b"\
            POST / HTTP/1.1\r\n\
            Upgrade: foobar\r\n\
            Connection: upgrade\r\n\
            Content-Length: 5\r\n\
            \r\n\
        ",
        )
        .await
        .unwrap();
    let mut head = vec![0; prefix.len()];
    client.read_exact(&mut head).await.unwrap();
    assert_eq!(head, prefix);
    client.write_all(b"hello").await.unwrap();
    let mut buf = head;
    client.read_to_end(&mut buf).await.unwrap();
    child.await.unwrap();

    let got = std::str::from_utf8(&buf).unwrap();
    let expected = "HTTP/1.1 101 Switching Protocols\r\n";
    assert!(got.starts_with(expected), "{:?}", got);
    assert!(got.ends_with("\r\n\r\nfoo=bar"), "{:?}", got);
}
