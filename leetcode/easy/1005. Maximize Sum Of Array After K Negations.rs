impl Solution {
    pub fn largest_sum_after_k_negations(nums: Vec<i32>, k: i32) -> i32 {
        let mut temp: Vec<i32> = nums.to_vec();
        temp.sort_by(|a, b| a.cmp(&b));
        // println!("starting : {:?}", temp);

        let mut temp_k: i32 = k;
        let mut idx: usize = 0;
        while idx < temp.len() && temp_k > 0 {
            temp_k -= 1;
            temp[idx] *= -1;
            // println!("{:?}", temp);
            if idx > 0 && idx < (temp.len() - 1) && temp[idx - 1] >= 0 && temp[idx + 1] >= 0 {
                break;
            }
            if idx < temp.len() - 1 && temp[idx + 1] < 0 {
                idx += 1;
            }
        }
        // println!("{:?} \nidx : {}\nk:{}", temp, idx, temp_k);
        if temp_k > 0 && idx >= 0 && idx <= (temp.len() - 1) {
            // find the smallest num
            while idx > 0 && temp[idx - 1] < temp[idx] {
                idx -= 1;
                if idx == 0 {
                    break;
                }
            }
            while idx < (temp.len() - 1) && temp[idx + 1] < temp[idx] {
                idx += 1;
            }
            // // println!("----");
            // // println!("{:?} -> {}", temp, temp[idx]);
            temp_k = (temp_k % 2);
            if temp_k == 1 && idx < temp.len() {
                temp[idx] *= -1
            }
        }

        let mut sum: i32 = 0;
        for i in 0..temp.len() {
            sum += temp[i];
        }

        sum
    }
}
