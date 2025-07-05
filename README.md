# README

Still an experiment.

https://ciurana-life.github.io/snake4/


## Develop
Install http server:
```bash
cargo install basic-http-server
```

Build
```
cargo build --target wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/snake4.wasm .
```

Run the server:
```bash
basic-http-server .
```


## IOS
https://macroquad.rs/articles/ios/
install xcode, and agree license.
```
mkdir MyGame.app
sudo xcode-select --switch /Applications/Xcode.app
cargo build --target aarch64-apple-ios --release
or
cargo build --target aarch64-apple-ios-sim --release
```
