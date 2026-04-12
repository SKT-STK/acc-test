set shell := ["cmd.exe", "/c"]
set dotenv-load := true

_setup:
  copy InputSimulator.dll target\\release\\InputSimulator.dll

build: _setup
  cargo build --release

run *ARGS: _setup
  cargo run --release -- {{ARGS}}

run_spa:
  just run (8 0x61 80) (25 0x62) (76 0x60) (92 0x62) [0x63 0x65 -5] [92 54.0] [8 56.4]

run_barcelona:
  just run (59 0x61) (68 0x62) [0x63 0x65 -5] [68 54.0] [78 56.4]
