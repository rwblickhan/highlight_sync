alias b := build

build:
  cargo build --release && cp target/release/sync_highlights ~/bin && cp target/release/sync_highlights.1 ~/man/man1
