
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
ls -h -s ./target/debug/breakout.exe

# wasm
echo "Build Wasm:"
# # rustup  target add      wasm32-unknown-unknown &
cargo   build --target  wasm32-unknown-unknown &
echo "Size Wasm:"
ls -h -s ./target/wasm32-unknown-unknown/debug/breakout.wasm
# echo "Run Wasm:"
# cargo install basic-http-server
# basic-http-server .

date
echo done
