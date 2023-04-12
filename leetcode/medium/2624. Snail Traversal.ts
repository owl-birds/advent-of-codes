declare global {
  interface Array<T> {
    snail(rowsCount: number, colsCount: number): number[][];
  }
}

Array.prototype.snail = function (
  rowsCount: number,
  colsCount: number
): number[][] {
  if (rowsCount * colsCount !== this.length) {
    return [];
  }
  const new_arr = [];
  for (let r = 0; r < rowsCount; r += 1) {
    new_arr.push(new Array(colsCount));
  }
  let is_down = true;
  let idx = 0;
  for (let i = 0; i < colsCount; i += 1) {
    if (is_down) {
      for (let r = 0; r < rowsCount; r += 1) {
        new_arr[r][i] = this[idx];
        idx += 1;
      }
    } else {
      for (let r = rowsCount - 1; r >= 0; r -= 1) {
        new_arr[r][i] = this[idx];
        idx += 1;
      }
    }
    is_down = !is_down;
  }
  return new_arr;
};

/**
 * const arr = [1,2,3,4];
 * arr.snail(1,4); // [[1,2,3,4]]
 */
