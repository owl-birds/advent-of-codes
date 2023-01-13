function subarraysDivByK(nums: number[], k: number): number {
  let count: number = 0;
  const prefix_sums: number[] = new Array(nums.length).fill(0);
  prefix_sums[0] = nums[0];
  for (let i = 1; i < nums.length; i += 1) {
    prefix_sums[i] = prefix_sums[i - 1] + nums[i];
    if ((prefix_sums[i] - prefix_sums[i - 1]) % k === 0) count += 1;
  }
  if (prefix_sums[0] % k === 0) count += 1;
  for (let i = 1; i < prefix_sums.length; i += 1) {
    for (let j = i; j < prefix_sums.length; j += 1) {
      if (i === j) {
        if (prefix_sums[i] % k === 0) count += 1;
        continue;
      }
      if ((prefix_sums[j] - prefix_sums[i - 1]) % k === 0) count += 1;
    }
  }
  return count;
}
const subarrays_div_k = (nums: number[], k: number): number => {
  const remainder_count: { [key: number]: number } = { 0: 1 };
  let result: number = 0;
  let curr_sum: number = 0;
  for (let i = 0; i < nums.length; i += 1) {
    curr_sum += nums[i];
    let remainder: number = curr_sum % k;
    if (remainder < 0) remainder += k;
    if (remainder_count[remainder]) {
      result += remainder_count[remainder];
      remainder_count[remainder] += 1;
      continue;
    }
    remainder_count[remainder] = 1;
  }
  return result;
};
let start: number = performance.now();
console.log(subarraysDivByK([-1, 2, 9], 2));
let end: number = performance.now();
console.log(`start: ${start}\nend: ${end}\ntime : ${end - start} milliseconds`);
console.log("BETTER");
start = performance.now();
console.log(subarrays_div_k([-1, 2, 9], 2));
end = performance.now();
console.log(`start: ${start}\nend: ${end}\ntime : ${end - start} milliseconds`);
