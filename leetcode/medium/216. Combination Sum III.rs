impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut nums: Vec<i32> = vec![];
        for num in 1..10 as i32 {
            nums.push(num);
        }
        // println!("{:?}", nums);
        let mut result: Vec<Vec<i32>> = vec![];
        let mut comb: Vec<i32> = vec![];
        Self::backtrack(&nums, &mut comb, k, &mut result, n, 0);
        result
    }
    pub fn backtrack(
        nums: &Vec<i32>,
        comb: &mut Vec<i32>,
        limit_len: i32,
        storage: &mut Vec<Vec<i32>>,
        target: i32,
        idx: usize,
    ) {
        // base case
        if target < 0 {
            return;
        }
        if target == 0 && comb.len() as i32 == limit_len {
            storage.push(comb.to_vec());
            return;
        }
        if comb.len() as i32 == limit_len {
            return;
        }
        // base case

        // RECURSE
        for i in idx..nums.len() {
            comb.push(nums[i]);
            Self::backtrack(nums, comb, limit_len, storage, target - nums[i], i + 1);
            comb.pop();
        }
    }
}
