
set -e # stop on first error

echo
echo "Build:"
cargo fmt
cargo build

# windows
# rustup  target add      x86_64-pc-windows-gnu
# cargo   build           --target  x86_64-pc-windows-gnu
cargo   build --release --target  x86_64-pc-windows-gnu &
cargo   run   --release --target  x86_64-pc-windows-gnu &

echo
echo "Size:"
ls -lh ./target/debug/breakout.exe

# wasm
# echo "build_wasm 21"
# # rustup  target add      wasm32-unknown-unknown &
cargo   build --target  wasm32-unknown-unknown &
# echo "Size Wasm:"
ls -lh ./target/wasm32-unknown-unknown/debug/breakout.wasm
# echo "Run:"
# cargo install basic-http-server
# basic-http-server .

date
echo done
