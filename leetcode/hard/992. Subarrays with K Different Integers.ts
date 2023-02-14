// TLE
function subarraysWithKDistinct(nums: number[], k: number): number {
    let count = 0;

    for (let i = 0; i < nums.length; i += 1){
        const unique_num: {[num: number]: boolean} = {};
        for (let j = i; j < nums.length; j += 1){
            if (!unique_num[nums[j]]){
                unique_num[nums[j]] = true;
            }
            if (Object.keys(unique_num).length === k) count += 1;
            if (Object.keys(unique_num).length > k) break; 
        }
    }

    return count; 
};
