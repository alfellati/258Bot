.PHONY: init

install-rust:
    @curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    @source $HOME/.cargo/env

install-make:
    @sudo apt update
    @sudo apt install make


.PHONY: build

build:
    cargo build --release


.PHONY: run
run:
    cargo run --release
