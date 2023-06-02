use std::collections::HashSet;
impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut o_result: i32 = original;
        let mut nums_set: HashSet<i32> = HashSet::new();
        for num in nums {
            nums_set.insert(num);
        }

        while nums_set.contains(&o_result) {
            o_result *= 2;
        }

        o_result
    }
}
