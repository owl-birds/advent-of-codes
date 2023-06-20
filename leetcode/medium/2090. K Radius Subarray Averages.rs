impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut avrg_sub_k: Vec<i32> = vec![];
        let sub_arr_length: i64 = (2 * k) as i64 + 1;
        if sub_arr_length as usize > nums.len() {
            // println!("{} {}", avrg_sub_k.len(), nums.len());
            for i in 0..nums.len() {
                avrg_sub_k.push(-1);
            }
            return avrg_sub_k;
        }
        // SLIDING WINDOW i THINK
        let mut temp_sum: i64 = 0;
        let mut idx: usize = k as usize;
        for i in 0..(k + 1) as usize {
            temp_sum += nums[idx + i] as i64;
            if i > 0 {
                avrg_sub_k.push(-1);
                temp_sum += nums[idx - i] as i64;
            }
        }
        avrg_sub_k.push((temp_sum / sub_arr_length) as i32);
        // println!("{}", temp_sum);
        idx += (k + 1) as usize;
        while idx < nums.len() {
            temp_sum += nums[idx] as i64;
            temp_sum -= nums[idx - sub_arr_length as usize] as i64;
            avrg_sub_k.push((temp_sum / sub_arr_length) as i32);
            idx += 1;
        }
        for i in 0..k {
            avrg_sub_k.push(-1);
        }
        avrg_sub_k
        // SLIDING WINDOW i THINK

        // // THE OVERCOMPLICATED SOLUTION
        // let mut idx: i32 = 0;

        // while idx < ((nums.len() as i32) - k) {
        //     if idx - k < 0 {
        //         avrg_sub_k.push(-1);
        //         idx += 1;
        //         continue;
        //     }
        //     let mut sum: i32 = 0;
        //     let mut temp_idx: usize = idx as usize;
        //     for i in 0..(k+1) as usize {
        //         sum += nums[temp_idx + i];
        //         if i > 0 {
        //             sum += nums[temp_idx - i];
        //         }
        //     }

        //     avrg_sub_k.push(sum/((k*2)+1));
        //     idx += 1;
        // }
        // while idx < nums.len() as i32 {
        //     avrg_sub_k.push(-1);
        //     idx += 1;
        // }
        // avrg_sub_k
        // // THE OVERCOMPLICATED SOLUTION
    }
}
