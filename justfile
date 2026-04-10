set shell := ["cmd.exe", "/c"]
set dotenv-load := true

_setup:
  copy InputSimulator.dll target\\release\\InputSimulator.dll

build: _setup
  cargo build --release

run: _setup
  cargo run --release -- (8 0x61 80) (25 0x62) (76 0x60) (92 0x62) [0x63 0x65 -5] [92 54.0] [0 50.0] [8 56.4]
