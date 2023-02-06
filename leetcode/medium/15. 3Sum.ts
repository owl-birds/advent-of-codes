function threeSum(nums: number[]): number[][] {
    const result3SumArr: number[][] = [];
    
    // two pointers BAD TWO POINTERS TLE
    // 308/312
    const temp_nums: number[] = [...nums].sort((a,b)=>a-b);
    const already_exist_2: {}[] = [];
    for (let i = 0; i <= temp_nums.length-3; i += 1){
        let left: number = i + 1;
        let right: number = temp_nums.length-1;

        while (left < right){
            const sum: number = temp_nums[i] + temp_nums[left] + temp_nums[right];
            if (sum === 0) {
                const temp_obj: {} = {};
                for (let num of [temp_nums[i], temp_nums[left], temp_nums[right]]){
                    if (!temp_obj[num]){
                        temp_obj[num] = 1;
                        continue;
                    }
                    temp_obj[num] += 1;
                }
                if (!is_in(temp_obj, already_exist_2)){
                    result3SumArr.push([temp_nums[i], temp_nums[left], temp_nums[right]]);
                    already_exist_2.push(temp_obj);
                }
            }
            if (sum > 0){
                right -= 1;
                continue;
            }
            left += 1;
        }
    }
    return result3SumArr;


    // TLE, REAL BAD SOLUTION, but hey AT LEAST IT WORK for a while
    // so much redudancy
    const already_exist: {}[] = [];
    for (let i = 0; i < nums.length; i += 1){
        for (let j = 0; j < nums.length; j += 1){
            if (i === j) continue;
            for (let k = 0; k < nums.length; k += 1){
                if (j === k || i === k) continue;
                const sum3: number = nums[i] + nums[j] + nums[k];
                if (sum3 === 0){
                    const temp_obj: {} = {};
                    // temp_obj[nums[i]] = true;
                    // temp_obj[nums[j]] = true;
                    // temp_obj[nums[k]] = true;
                    for (let num of [nums[i], nums[j], nums[k]]){
                        if (!temp_obj[num]){
                            temp_obj[num] = 1;
                            continue;
                        }
                        temp_obj[num] += 1;
                    }
                    // result3SumArr.push([nums[i],nums[j],nums[k]]);
                    if (!is_in(temp_obj, already_exist)){
                        console.log(temp_obj)
                        result3SumArr.push([nums[i],nums[j],nums[k]]);
                        already_exist.push(temp_obj);
                    }
                }
            }
        }
    }
    return result3SumArr;
};

const is_in = (obj_1: {}, obj_arr: {}[]): boolean=>{
    for (let obj of obj_arr){
        let count: number = 0;
        for (let key of Object.keys(obj_1)){
            if (obj[key] === obj_1[key]) count += 1;
        }
        if (count === Object.keys(obj).length) return true;
    }
    return false;
}
