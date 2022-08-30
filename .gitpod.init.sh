rustup target add x86_64-pc-windows-gnu
sudo apt update
sudo apt install mingw-w64 -y
sudo apt upgrade -y
cargo watch -x 'build --target x86_64-pc-windows-gnu'