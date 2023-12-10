impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let mut key_idxs: Vec<usize> = vec![];
        for i in 0..nums.len() {
            if nums[i] == key {
                key_idxs.push(i);
            }
        }

        let mut ans: Vec<i32> = vec![];
        for i in 0..nums.len() {
            let mut is_k_distance: bool = false;
            for j in 0..key_idxs.len() {
                if i32::abs(i as i32 - key_idxs[j] as i32) <= k {
                    is_k_distance = true;
                    break;
                }
            }
            if is_k_distance {
                ans.push(i as i32);
            }
        }

        ans
    }
}
