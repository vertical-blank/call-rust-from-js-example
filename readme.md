
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
wasm-pack build --target nodejs --release --out-dir ./wasm-target
```


## [WIP] Cross compile

```
cargo install cross
```

## Benchmark

```
$ time node js/wasm.js 100
354224848179261915075

real    0m0.062s
user    0m0.118s
sys     0m0.000s
$ time node js/wasm.js 100
354224848179261915075

real    0m0.060s
user    0m0.104s
sys     0m0.021s
$ time node js/native.js 100
354224848179261915075

real    0m0.147s
user    0m0.251s
sys     0m0.043s
$ time node js/native.js 100
354224848179261915075

real    0m0.141s
user    0m0.238s
sys     0m0.030s
$ time target/debug/runner 100
354224848179261915075

real    0m0.002s
user    0m0.002s
sys     0m0.000s
$ time target/debug/runner 100
354224848179261915075

real    0m0.002s
user    0m0.002s
sys     0m0.000s
```


