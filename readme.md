
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

## [WIP] Cross compile

```
cargo install cross
```

