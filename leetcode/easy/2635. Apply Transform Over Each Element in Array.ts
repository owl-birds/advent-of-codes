function map(arr: number[], fn: (n: number, i: number) => number): number[] {
  const new_arr: number[] = [];
  for (let i = 0; i < arr.length; i += 1) {
    new_arr.push(fn(arr[i], i));
  }
  return new_arr;
}
