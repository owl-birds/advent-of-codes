function singleNonDuplicate(nums: number[]): number {    
    // O(N) ;;;;; O(1)
    // let count = 0;
    // let temp_num = nums[0];
    // for (let num of nums){
    //     if (temp_num === num){
    //         count += 1;
    //         continue;
    //     }
    //     if (count === 1) return temp_num;
    //     count = 1;
    //     temp_num = num;
    // }
    // return temp_num;

    // a little bit optimized 
    let left = 0;
    let left_num = nums[left];
    let count_left = 0;
    
    let right = nums.length-1;  
    let right_num = nums[right];
    let count_right = 0;

    while (left <= right){
        if (left_num === nums[left]){
            count_left += 1;
        }else{
            if (count_left === 1) return left_num;
            count_left = 1;
            left_num = nums[left];
        }
        if (right_num === nums[right]){
            count_right += 1
        }else{
            if (count_right === 1) return right_num;
            count_right = 1;
            right_num = nums[right]
        }
        left += 1;
        right -= 1;
    }
    return left_num;

};
