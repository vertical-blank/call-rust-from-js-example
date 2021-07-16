const wasm = require("../rust/wasm/wasm-target/wasm");

wasm.fibs(10);
const fib = wasm.fib(100);
console.log(fib);
