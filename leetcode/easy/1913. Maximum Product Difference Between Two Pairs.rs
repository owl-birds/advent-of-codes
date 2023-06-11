impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let mut max_first_idx: usize = 0;

        for i in 0..nums.len() {
            if nums[max_first_idx] < nums[i] {
                max_first_idx = i
            }
        }

        let mut max_second_idx: usize = if max_first_idx == 0 {
            1
        } else {
            max_first_idx - 1
        };

        for i in 0..nums.len() {
            if nums[max_second_idx] < nums[i] && max_first_idx != i {
                max_second_idx = i
            }
        }

        let mut min_first_idx: usize = 0;

        for i in 0..nums.len() {
            if nums[min_first_idx] > nums[i] {
                min_first_idx = i
            }
        }

        let mut min_second_idx: usize = if min_first_idx == 0 {
            1
        } else {
            min_first_idx - 1
        };

        for i in 0..nums.len() {
            if nums[min_second_idx] > nums[i] && min_first_idx != i {
                min_second_idx = i
            }
        }

        // println!("{} {} {} {}", nums[max_first_idx], nums[max_second_idx], nums[min_first_idx], nums[min_second_idx]);

        (nums[max_first_idx] * nums[max_second_idx]) - (nums[min_first_idx] * nums[min_second_idx])

        // let mut temp_nums: Vec<i32> = nums.clone();

        // temp_nums.sort();

        // (temp_nums[temp_nums.len()-1]*temp_nums[temp_nums.len()-2]) - (temp_nums[1]*temp_nums[0])
    }
}
