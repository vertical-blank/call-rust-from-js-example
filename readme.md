
# Call Rust lib from JavaScript

## Build Rust lib

```bash
cd rust
cargo build
```

## Call from JavaScript

```
cd js
npm i
cp ../rust/target/debug/libfoo.so .
node index.js
```

## Build Rust into WASM

```
rustup target add wasm32-unknown-unknown
cargo install wasm-pack 
wasm-pack build --target nodejs --release --out-dir ./wasm
```


## [WIP] Cross compile

```
cargo install cross
```

