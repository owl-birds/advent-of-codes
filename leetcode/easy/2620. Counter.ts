function createCounter(n: number): () => number {
  let count: number = n - 1;
  // let count: number = n;
  return function () {
    // let prev_count = count;
    count += 1;
    return count;
  };
}

/**
 * const counter = createCounter(10)
 * counter() // 10
 * counter() // 11
 * counter() // 12
 */
