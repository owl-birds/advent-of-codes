function curry(fn: Function): Function {
  return function curried(...args: number[]) {
    if (fn.length === args.length) {
      return fn(...args);
    }
    return (...new_args: number[]) => {
      return curried(...args, ...new_args);
    };
  };
}

/**
 * function sum(a, b) { return a + b; }
 * const csum = curry(sum);
 * csum(1)(2) // 3
 */
