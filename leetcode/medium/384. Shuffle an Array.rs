/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(nums);
 * let ret_1: Vec<i32> = obj.reset();
 * let ret_2: Vec<i32> = obj.shuffle();
 */
use rand::Rng;
use std::collections::HashSet;

#[derive(Debug)]
struct Solution {
    ori_nums: Vec<i32>,
    // curr_nums: Vec<i32>,
    length: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            ori_nums: nums.to_vec(),
            // curr_nums: nums.to_vec(),
            length: nums.len(),
        }
    }

    fn reset(&mut self) -> Vec<i32> {
        // self.curr_nums = self.ori_nums.to_vec();

        // self.curr_nums.to_vec()
        self.ori_nums.to_vec()
    }

    fn shuffle(&mut self) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        // println!("{}", rng.gen_range(0, self.curr_nums.len()));
        let mut idxs: HashSet<usize> = HashSet::new();
        while idxs.len() != self.length {
            let rand_idx: usize = rng.gen_range(0, self.length);
            if !idxs.contains(&rand_idx) {
                idxs.insert(rand_idx);
            }
        }
        // println!("{:?}", idxs);
        let mut new_nums: Vec<i32> = vec![];
        for i in idxs.iter() {
            new_nums.push(self.ori_nums[*i]);
        }
        // println!("{:?}", new_nums);
        // vec![]
        new_nums
    }
}
