set shell := ["cmd.exe", "/c"]
set dotenv-load := true

build:
  cargo build --release

run:
  cargo run --release -- (8 0x61 80) (25 0x62) (76 0x60) (92 0x62) [0x63 0x65] [92 54.0] [0 50.0] [8 56.0]
