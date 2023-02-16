// still not constant space 
function firstMissingPositive(nums: number[]): number {
    // const max_num = Math.max(...nums);
    
    const temp_nums: {[num: number]: boolean} = {};
    for (let num of nums){
        if (temp_nums[num] === undefined) temp_nums[num] = true;
    }

    let temp_num = 1;
    while (true){
        if (temp_nums[temp_num] === undefined) {
            return temp_num;
        }
        temp_num += 1;
    }
    
    // TLE
    // n^2
    while (true){ // not recomended
        let is_found = false;
        for (let num of nums){
            if (num <= 0) continue;
            if (num === temp_num){
                is_found = true;
                break;
            }
        }        
        if (!is_found) return temp_num;
        temp_num += 1;
    }


    while (true){ // not recomended
        if (nums.indexOf(temp_num) === -1) return temp_num; // this part is the problem ; i think 
        temp_num += 1;
    }
    
    // not work
    // for (let i = 1; i < max_num; i += 1){
    //     if (nums.indexOf(i) === -1) return i;
    // }
};
