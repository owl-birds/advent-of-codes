function threeSumClosest(nums: number[], target: number): number {
    if (nums.length <= 3){
        return nums.reduce((prev_val, curr_num)=>prev_val+curr_num,0);
    }
    // two pointer and sorting solution
    const temp_nums: number[] = [...nums].sort((a,b)=>a-b);
    let result_sum: number = temp_nums[0] + temp_nums[1] + temp_nums[2];

    for (let i = 0; i <= temp_nums.length-3; i += 1){
        let left = i+1;
        let right = temp_nums.length-1;
        while (left < right){
            let temp_sum: number = temp_nums[i] + temp_nums[left] + temp_nums[right];
            if (Math.abs(temp_sum-target) <= Math.abs(result_sum-target)){
                result_sum = temp_sum;
                if (result_sum === target) return result_sum;
            } 
            if (temp_sum > target){
                right -= 1;
                continue;
            }
            left += 1;
        }
    }

    return result_sum;
};
