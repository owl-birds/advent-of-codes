function numSubarraysWithSum(nums: number[], goal: number): number {
  // slow algo
  //   const prefix_sum: number[] = new Array(nums.length).fill(0);
  //   prefix_sum[0] = nums[0];
  //   for (let i = 1; i < nums.length; i += 1) {
  //     prefix_sum[i] = prefix_sum[i - 1] + nums[i];
  //   }
  //   let count: number = 0;
  //   for (let i = 0; i < prefix_sum.length; i += 1) {
  //     for (let j = i; j < prefix_sum.length; j += 1) {
  //       const sum: number = prefix_sum[j] - prefix_sum[i] + nums[i];
  //       if (sum > goal) break;
  //       if (sum === goal) count += 1;
  //     }
  //   }
  //   return count;
  /// BETTER ONE
  let count: number = 0;
  let curr_sum: number = 0;
  const prefix_sum: { [index: number]: number } = {};
  for (let i = 0; i < nums.length; i += 1) {
    curr_sum += nums[i];
    if (curr_sum === goal) count += 1;
    if (prefix_sum[curr_sum - goal]) {
      count += prefix_sum[curr_sum - goal];
    }
    if (!prefix_sum[curr_sum]) {
      prefix_sum[curr_sum] = 1;
      continue;
    }
    prefix_sum[curr_sum] += 1;
  }
  return count;
}
