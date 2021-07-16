def fib(n: Int): BigInt = {
  @annotation.tailrec
  def loop(n: Int, a: BigInt, b: BigInt): BigInt =
    if (n == 0) a
    else loop(n - 1, b, a + b)
  loop(n, 0, 1)
}

println(fib(100))
