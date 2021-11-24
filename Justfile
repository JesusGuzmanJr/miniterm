INSTALL_PATH := "/usr/local/bin"
BIN_NAME := "miniterm"

_default:
    @just --list

build:
	@cargo build --release

clean:
	@cargo clean

check:
	@cargo check

clippy:
	@cargo clippy

install: build
	sudo cp target/release/{{BIN_NAME}} {{INSTALL_PATH}}

uninstall:
	sudo rm {{INSTALL_PATH}}/{{BIN_NAME}}