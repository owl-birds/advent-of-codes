impl Solution {
    pub fn maximum_count(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] > 0 {
                right = mid;
                continue;
            }
            left = mid + 1;
        }
        let pos_count = if nums[left] == 0 {
            0
        } else {
            (nums.len() - left) as i32
        };
        // println!("{}", left);

        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < 0 {
                left = mid + 1;
                continue;
            }
            right = mid;
        }

        let neg_count = if left == nums.len() - 1 {
            (left + 1) as i32
        } else {
            left as i32
        };
        // println!("{} {} {}", pos_count, neg_count, left);
        if pos_count > neg_count {
            return pos_count;
        }
        neg_count
    }
}
