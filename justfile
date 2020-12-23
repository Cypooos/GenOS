# To change used tools edit .env file
# For a deatailed description of justfiles go to https://github.com/casey/just#quick-start

# Standard justfile recipe if there no arguments
_default:
	@just --list

# Clean all build files
clean:
	cargo clean

# Test os with target system
test:
	cargo test --target targets/x86_64-genos.json
	cargo bootimage
	$QEMU_RUN_COMMAND

# Build crate for target
build:
	cargo build --target targets/x86_64-genos.json
	cargo bootimage

# Install all necessary rust tools for running
install:
	rustup toolchain install nightly
	rustup update nightly --force
	rustup toolchain add nightly
	rustup override set nighly
	rustup component add rust-src
	cargo +nightly build
	rustup component add llvm-tools-preview
	cargo install bootimage

# Run qemu 
run: build
	$QEMU_RUN_COMMAND

