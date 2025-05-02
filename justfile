alias b := build

build:
  cargo build --release && cp target/release/highlight_sync ~/bin && cp target/release/highlight_sync.1 ~/man/man1
