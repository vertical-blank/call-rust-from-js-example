const wasm = require("../rust/wasm/wasm-target/wasm");

const n = process.argv[2];
const fib = wasm.fib(n);
console.log(fib);
