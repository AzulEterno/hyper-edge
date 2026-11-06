//! Checked builder updates preserve a valid setting on invalid input.
use crate::server::conn::http2::Builder;
#[test]
fn write_batch_size_validates_before_mutating() {
    let mut builder = Builder::new(());
    assert!(builder.h2_builder.write_batch_size.is_none());
    builder.write_batch_size(Some(131144)).unwrap();
    for invalid in [0, 32785, usize::MAX] {
        assert_eq!(
            builder.write_batch_size(Some(invalid)).unwrap_err().kind(),
            std::io::ErrorKind::InvalidInput
        );
        assert_eq!(builder.h2_builder.write_batch_size.unwrap().get(), 131144);
    }
    builder.write_batch_size(Some(32786)).unwrap();
    assert_eq!(builder.h2_builder.write_batch_size.unwrap().get(), 32786);
    builder.write_batch_size(None).unwrap();
    assert!(builder.h2_builder.write_batch_size.is_none());
}
