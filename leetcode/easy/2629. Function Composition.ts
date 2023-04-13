type F = (x: number) => number;

function compose(functions: F[]): F {
  return function (x: number): number {
    if (functions.length === 0) {
      return x;
    }
    let result: number = x;
    for (let i = functions.length - 1; i >= 0; i -= 1) {
      result = functions[i](result);
    }
    return result;
  };
}

/**
 * const fn = compose([x => x + 1, x => 2 * x])
 * fn(4) // 9
 */
