const ref = require('ref-napi');
const ffi = require('ffi-napi');

const libDefs = ffi.Library(`${__dirname}/../rust/target/debug/libnative`, {
  fibs: [
    ref.types.int,
    [ref.types.int32]
  ],
  fib: [
    ref.types.CString,
    [ref.types.int32]
  ]
});

libDefs.fibs(10);
const fib = libDefs.fib(100);
console.log(fib);
