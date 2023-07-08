impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        // mean = (sum_arr_m + sum_arr_n) / (m+n)
        // mean (m+n) = (sum_arr_m+sum_arr_n)
        // find sum_n
        let m: i32 = rolls.len() as i32;
        let mut sum_m: i32 = 0;
        for num in rolls {
            sum_m += num;
        }
        let sum_n: i32 = ((m + n) * mean) - sum_m;
        // println!("sum_m:{}\nsum_n:{}", sum_m, sum_n);
        // if sum_n%n != 0 && sum_n/n > 6 {
        //     println!("!!!!!");
        //     return vec![];
        // }
        // let mut two_one_vec: Vec<i32> = Self::generate_one_two_vec(sum_n);
        // let mut two_one_vec: Vec<i32> = Self::gen_nums_based_on_sum(sum_n, n, 1, 6);
        // println!("temp n vec:{:?}", two_one_vec);

        let mut result: Vec<i32> = Self::gen_nums_based_on_sum(sum_n, n, 1, 6);
        // println!("{:?}", result);
        result
    }
    pub fn generate_one_two_vec(sum: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut temp_sum = sum;
        if temp_sum % 2 != 0 {
            temp_sum -= 1;
            result.push(1);
        }
        for i in 0..(temp_sum / 2) {
            result.push(2);
        }
        result
    }
    pub fn gen_nums_based_on_sum(sum: i32, size: i32, min_num: i32, max_num: i32) -> Vec<i32> {
        // check can be generated
        if (sum % size > 0 && sum / size == 6)
            || sum / size > max_num
            || sum / size == 0
            || sum / size < min_num
        {
            return vec![];
        }
        // check can be generated

        let mut result: Vec<i32> = vec![];
        let base_num: i32 = sum / size;
        let mut left_over: i32 = sum % size;
        for i in 0..size {
            result.push(base_num);
        }
        let mut idx: usize = 0;
        while idx < result.len() && left_over > 0 {
            if result[idx] >= 6 {
                idx += 1;
                continue;
            }
            result[idx] += 1;
            left_over -= 1;
        }
        result
    }
}
