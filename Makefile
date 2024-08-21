rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format-check:
	cargo fmt --quiet --manifest-path main/Cargo.toml

lint:
	cargo clippy --quiet --manifest-path main/Cargo.toml

test:
	cargo test --quiet --manifest-path main/Cargo.toml

run:
	cargo run --manifest-path main/Cargo.toml

build-release:
	cargo build --release --manifest-path main/Cargo.toml

all: format lint test run
