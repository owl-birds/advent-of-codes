impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len()-1;

        while start < end{
            let mid = (start+end)/2;
            if nums[mid] < target{
                start = mid + 1;
                continue;
            }
            end = mid;
        }
        if nums[start] != target{return -1}
        start as i32
    }
}
