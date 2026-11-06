# hyper-edge: purpose and attribution

This repository is an independent development fork of
[hyper](https://github.com/hyperium/hyper), starting from upstream
1.12.0. Its purpose is to evaluate and carry selected fixes and improvements
while we eagerly wait for them to land upstream and appear in official releases.

We do not want to take credit for the upstream project or for fixes written by
other contributors. Credit belongs to the original authors and pull-request
authors. Our role here is integration, testing, and temporary maintenance;
renaming a package or carrying a patch does not make that work our own.

Preserve upstream copyright notices, licenses, author attribution, and commit
history. Record the original pull request and commit when carrying a change.
When an official release includes a carried fix, update the upstream base and
remove the redundant local patch. This is not an official Hyperium release or
an endorsement by its maintainers.

## Package identity

The Cargo package is named `hyper-edge`. Its Rust library target remains `hyper`
so existing source imports and upstream tests can retain their familiar names.
The package version currently matches the upstream base; the distinct package
name identifies this fork. Upstream documentation links describe the upstream API.

These development checkouts use sibling path dependencies where needed:
`hyper-util-edge` uses `hyper-edge`, and `hyper-edge` uses `hyper-h2-edge`.
Dependency aliases preserve the source names `hyper` and `h2`. Consumers must
select the renamed packages explicitly; they do not automatically replace the
original crates elsewhere in a dependency graph.

## Carried local batching patch

This fork carries our optional server DATA batching implementation via `hyper::server::conn::http2::Builder::write_batch_size(Option<usize>)`.
This is our local patch, not an implementation or cherry-pick of
[h2 #903](https://github.com/hyperium/h2/pull/903). Upstream authors retain
credit for the underlying projects.

`None` disables optional DATA-copy batching. Other carried upstream fixes still
apply. A checked target must be between 32786 and
`isize::MAX` encoded bytes. The h2 encoder grows lazily on the first full 16 KiB
DATA frame and copies those payloads into the batch; other frame sizes retain
the upstream path. The target is not a total connection memory limit. Peer
frame sizes, flow control and flush ownership remain unchanged.

## Upstream correctness fix

Carry [hyperium/hyper#4233](https://github.com/hyperium/hyper/pull/4233),
commit `7042ee2961e6edd0f8ca364ef1741624572f8aca`: flush buffered HTTP/1
responses before upgrade handoff. Runtime logic and regression originate from
the upstream PR author; the regression lives under `src/tests/upgrade_flush.rs`.
