const ref = require('ref-napi');
const ffi = require('ffi-napi');

const libDefs = ffi.Library(`${__dirname}/libfoo`, {
  hello: [
    ref.types.int,
    [ref.types.int32]
  ]
});

const hello = libDefs.hello(10);
console.log(hello);
