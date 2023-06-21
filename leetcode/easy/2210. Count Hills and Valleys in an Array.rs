impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut count: i32 = 0;
        let mut i: usize = 1;
        while i < nums.len() - 1 {
            let mut temp_i: usize = i;
            let mut what: u8 = 0;
            // o: neither, 1: hill, 2: valley
            while temp_i > 0 {
                temp_i -= 1;
                if nums[i] < nums[temp_i] {
                    what = 2; // valley
                    break;
                }
                if nums[i] > nums[temp_i] {
                    what = 1; // hill
                    break;
                }
                what = 0;
                if temp_i == 0 {
                    break;
                }
            }
            if what == 0 {
                i += 1;
                continue;
            }
            let mut temp_i: usize = i;
            while temp_i < nums.len() {
                temp_i += 1;
                if what == 2 && nums[i] < nums[temp_i] {
                    count += 1;
                    i = temp_i;
                    break;
                }
                if what == 1 && nums[i] > nums[temp_i] {
                    count += 1;
                    i = temp_i;
                    break;
                }
                what = 0;
            }
            if what == 0 {
                i += 1;
            }
        }

        count
    }
}
