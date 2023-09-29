impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 1;
        }

        // VALLEY AND PEAK :: HINT
        let mut count: i32 = 0; // starting point

        // finding the starting point is valley or not
        let mut is_valley: bool = nums[0] < nums[1];
        let mut idx: usize = 0;
        while idx < nums.len() - 1 && nums[idx] == nums[idx + 1] {
            idx += 1;
        }
        if idx != 0 && idx < nums.len() - 1 {
            is_valley = nums[idx] < nums[idx + 1];
        }
        // finding the starting point is valley or not

        println!("starting point is_valley : {}", is_valley);

        let mut idx: usize = 0;

        // my idea is finding the start of any valley or any peak, then iterate till the peak and the bottom(the end of the valley) of the valley
        while idx < nums.len() {
            if is_valley {
                while idx < nums.len() {
                    idx += 1;
                    if idx < nums.len() && nums[idx] > nums[idx - 1] {
                        break;
                    }
                }
                count += 1;
                is_valley = false;
                continue;
            }
            while idx < nums.len() {
                idx += 1;
                if idx < nums.len() && nums[idx] < nums[idx - 1] {
                    break;
                }
            }
            is_valley = true;
            count += 1;
        }

        // let mut diffs: Vec<i32> = vec![];
        // for i in 1..nums.len() {
        //     diffs.push(nums[i]-nums[i-1]);
        // }
        // println!("{:?}", diffs);

        count
    }
}
