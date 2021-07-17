
function fib(n) {
  const loop = (n, a, b) => {
    if (n == 0) return BigInt(a);
    else return loop(n - 1, b, a + b);
  }
  return loop(n, BigInt(0), BigInt(1));
}

const n = process.argv[2];
const f = fib(n);
console.log(f);
