@echo off

if "%~1"=="" goto :build_run
if "%~1"=="b" goto :build
if "%~1"=="r" goto :run
if "%~1"=="br" goto :build_run
if "%~1"=="t" goto :test
if "%~1"=="i" goto :install
if "%~1"=="n" exit
if "%~1"=="c" goto :clean

:clean
cargo clean
exit

:test
cargo xtest --target targets/x86_64-genos.json
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-genos/debug/bootimage-rust_genos.bin
exit


:build
cargo build --target targets/x86_64-genos.json
cargo bootimage
exit

:install
rustup toolchain install nightly
rustup toolchain add nightly
cargo +nightly build
rustup component add rust-src
rustup update nightly --force
cargo install bootimage

:build_run
cargo build --target targets/x86_64-genos.json
cargo bootimage

:run
qemu-system-x86_64 -drive format=raw,file=target/x86_64-genos/debug/bootimage-rust_genos.bin
exit


rem Linux : cargo rustc -- -C link-arg=-nostartfiles
rem Windows : cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
rem macOS : cargo rustc -- -C link-args="-e __start -static -nostartfiles"
