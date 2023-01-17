function minOperations(n: number): number {
  if (n === 1) return 0;
  const arr: number[] = new Array(n);
  for (let i = 0; i < n; i += 1) {
    arr[i] = 2 * i + 1;
  }
  // making the first and the last one the same number first
  const mid_num: number = (arr[0] + arr[n - 1]) / 2;
  // all the number will be the same as mid_num
  let count_operation: number = 0;
  for (let i = 0; i < arr.length / 2; i += 1) {
    count_operation += mid_num - arr[i];
  }
  return count_operation;
}
