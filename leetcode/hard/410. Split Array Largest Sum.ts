function splitArray(nums: number[], k: number): number {
  const max: number = Math.max(...nums);
  const sum: number = nums.reduce(
    (prev_sum, curr_num) => prev_sum + curr_num,
    0
  );
  // max ::: the smllest possible sum of an subarray
  // sum ::: the largest possible sum of an subarray

  // BINARY SEARCH : 100% faster
  let start: number = max;
  let end: number = sum;
  while (start < end) {
    const mid: number = Math.floor((start + end) / 2);
    if (is_feasible(mid, nums, k)) {
      end = mid;
      continue;
    }
    start = mid + 1;
  }
  return start;

  // linear : TLE
  for (let sum_increase = max; sum_increase <= sum; sum_increase += 1) {
    if (is_feasible(sum_increase, nums, k)) return sum_increase;
  }
}

const is_feasible = (
  group_sum: number,
  nums: number[],
  group: number
): boolean => {
  let temp_sum: number = 0;
  let count_group: number = 1;
  for (let i = 0; i < nums.length; i += 1) {
    temp_sum += nums[i];
    if (temp_sum > group_sum) {
      temp_sum = nums[i];
      count_group += 1;
      if (count_group > group) return false;
    }
  }
  return true;
};
