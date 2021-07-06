const wasm = require("../rust-wasm/wasm/foo");

const hello = wasm.hello(10);
console.log(hello);
