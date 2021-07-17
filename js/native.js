const ref = require('ref-napi');
const ffi = require('ffi-napi');

const libDefs = ffi.Library(`${__dirname}/../rust/target/release/libnative`, {
  fib: [
    ref.types.CString,
    [ref.types.int32]
  ]
});

const n = process.argv[2];
const fib = libDefs.fib(n);
console.log(fib);
