function minSubArrayLen(target: number, nums: number[]): number {
  // REAL SLOW SOLUTION
  //   const prefix_sum: number[] = [];
  //   prefix_sum[0] = nums[0];
  //   for (let i = 1; i < nums.length; i += 1) {
  //     prefix_sum[i] = prefix_sum[i - 1] + nums[i];
  //   }

  //   //
  //   let min_length: number = Infinity;
  //   for (let i = 0; i < prefix_sum.length; i += 1) {
  //     for (let j = i; j < prefix_sum.length; j += 1) {
  //       if (j - i + 1 > min_length) break;
  //       if (prefix_sum[j] - prefix_sum[i] + nums[i] >= target) {
  //         if (j - i < min_length) {
  //           min_length = j - i + 1;
  //         }
  //       }
  //     }
  //   }

  //     return min_length === Infinity ? 0 : min_length;

  // BETTER ONE : SLIDING WINDOW
  let start: number = 0;
  let end: number = 0;
  let min_length: number = Infinity;
  let curr_sum: number = 0;
  while (end < nums.length) {
    if (curr_sum < target) curr_sum += nums[end];
    end += 1;
    while (curr_sum >= target) {
      if (end - start < min_length) {
        min_length = end - start;
      }
      curr_sum -= nums[start];
      start += 1;
    }
  }
  return min_length === Infinity ? 0 : min_length;
}
