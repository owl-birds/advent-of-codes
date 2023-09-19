use std::collections::HashSet;
impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        // You must solve the problem without modifying the array nums and uses only constant extra space.

        // // not constant space
        // let mut num_set: HashSet<i32> = HashSet::new();
        // for i in 0..nums.len() {
        //     if !num_set.contains(&nums[i]) {
        //         num_set.insert(nums[i]);
        //         continue;
        //     }
        //     return nums[i];
        // }
        // // not constant space

        // constant space

        // Constraints:
        // - 1 <= n <= 105
        // - nums.length == n + 1
        // - 1 <= nums[i] <= n
        // - All the integers in nums appear only once except for precisely one integer which appears two or more times.

        // let mut slow: i32 = nums[0];
        let mut slow: i32 = nums[nums[0] as usize];
        // let mut fast: i32 = nums[0];
        let mut fast: i32 = nums[nums[nums[0] as usize] as usize];

        // println!("slow:{}->fast:{}", slow, fast);
        while slow != fast {
            // println!("slow:{}->fast:{}", slow, fast);
            slow = nums[slow as usize];
            fast = nums[nums[fast as usize] as usize];
        }
        // println!("final=>slow:{}->fast:{}", slow, fast);
        fast = nums[0];
        while slow != fast {
            slow = nums[slow as usize];
            fast = nums[fast as usize];
        }

        fast
    }
}
