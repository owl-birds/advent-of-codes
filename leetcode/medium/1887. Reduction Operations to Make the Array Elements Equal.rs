use std::collections::HashMap;
// use std::collections::HashSet;

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        // let mut temp: Vec<i32> = nums.to_vec();
        let mut temp: Vec<i32> = vec![];
        let mut nums_freq: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            if let Some(freq) = nums_freq.get_mut(&nums[i]) {
                *freq += 1;
            } else {
                temp.push(nums[i]);
                nums_freq.insert(nums[i], 1);
            }
        }
        temp.sort_by(|a, b| a.cmp(&b));
        // println!("nums:{:?}\nmap:{:?}", temp, nums_freq);

        let mut operations: i32 = 0;
        let mut idx: usize = temp.len();
        // let mut nums_set: HashSet<i32> = HashSet::new();
        let mut count: usize = 0;

        while idx >= 1 {
            idx -= 1;
            // if nums_set.len() == (nums_freq.len()-1) {break}
            if count == (nums_freq.len() - 1) {
                break;
            }
            // if nums_set.contains(&temp[idx]) {continue}
            // nums_set.insert(temp[idx]);
            // println!("--{}", operations);
            if let Some(freq) = nums_freq.get(&temp[idx]) {
                operations += (*freq * (idx as i32));
            }
            count += 1;
        }

        // println!("{:?}", temp);

        operations
    }
}
