
set -e # stop on first error

echo
echo "Build:"
cargo fmt
cargo build

# windows
rustup  target add      x86_64-pc-windows-gnu
cargo   build --target  x86_64-pc-windows-gnu

# linux
# rustup  target add      x86_64-unknown-linux-gnu
# cargo   build --target  x86_64-unknown-linux-gnu

# wasm
rustup  target add      wasm32-unknown-unknown
cargo   build --target  wasm32-unknown-unknown

echo
echo "Size:"
#./target/debug/breakout.exe
ls -lh ./target/debug/breakout.exe
ls -lh ./target/wasm32-unknown-unknown/debug/breakout.wasm

echo
echo "Run:"
cargo install basic-http-server
basic-http-server .
