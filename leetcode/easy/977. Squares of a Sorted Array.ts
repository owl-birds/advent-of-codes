function sortedSquares(nums: number[]): number[] {
  // let tempNums: number[] = [...nums]
  // tempNums = tempNums.map((num)=>num**2);
  // return tempNums.sort((a,b)=>a-b);

  const squared_nums: number[] = new Array(nums.length);
  let temp_idx: number = nums.length - 1;
  let left: number = 0;
  let right: number = nums.length - 1;
  while (left <= right) {
    if (left === right) {
      // squared_nums.unshift(nums[left] * nums[left]);
      squared_nums[temp_idx] = nums[left] * nums[left];
      temp_idx -= 1;
      break;
    }
    if (Math.abs(nums[left]) > Math.abs(nums[right])) {
      // squared_nums.unshift(nums[left] * nums[left]);
      squared_nums[temp_idx] = nums[left] * nums[left];
      temp_idx -= 1;
      left += 1;
      continue;
    }
    if (Math.abs(nums[left]) < Math.abs(nums[right])) {
      squared_nums[temp_idx] = nums[right] * nums[right];
      temp_idx -= 1;
      right -= 1;
      continue;
    }
    // if the same
    squared_nums[temp_idx] = nums[right] * nums[right];
    temp_idx -= 1;
    right -= 1;
  }
  return squared_nums;
}
