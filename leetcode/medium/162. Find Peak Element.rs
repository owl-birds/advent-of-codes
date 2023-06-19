impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len() - 1;

        while left < right {
            let mid = (left + right) / 2;
            if mid == 0 && nums[mid] > nums[mid + 1] {
                return 0;
            } else if mid == 0 && nums[mid] < nums[mid + 1] {
                return (mid + 1) as i32;
            }
            if nums[mid - 1] < nums[mid] && nums[mid] > nums[mid + 1] {
                return mid as i32;
            }
            if nums[mid - 1] > nums[mid] {
                right = mid;
                continue;
            }
            left = mid + 1;
        }

        left as i32
    }
}
