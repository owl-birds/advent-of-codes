use std::collections::HashSet;
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let mut result: String = String::new();
        let mut curr_idx: usize = 0;
        for i in 0..nums.len() {
            let temp: Vec<char> = nums[i].chars().collect();
            if temp[curr_idx] == '1' {
                result.push('0');
            } else {
                result.push('1');
            }
            curr_idx += 1;
        }

        // let bins: Vec<char> = vec!['0', '1'];
        // let mut bins_set: HashSet<String> = HashSet::new();
        // for num in &nums {
        //     bins_set.insert(num.to_string());
        // }
        // // println!("{:?}", bins_set);

        // Self::find_backtrack(&bins, &bins_set, &mut String::new(), &mut result, nums[0].len());

        result
    }
    pub fn find_backtrack(
        bins: &Vec<char>,
        bins_set: &HashSet<String>,
        curr_bin: &mut String,
        result: &mut String,
        length: usize,
    ) {
        // if curr_bin.len() > length {
        //     return;
        // }
        if curr_bin.len() == length && !bins_set.contains(curr_bin) {
            result.insert_str(0, &curr_bin[..]);
        }
        if result.len() != 0 {
            return;
        }
        // println!("{:?}", curr_bin);
        for i in 0..length {
            for j in 0..bins.len() {
                curr_bin.push(bins[i]);
                Self::find_backtrack(bins, bins_set, curr_bin, result, length);
                if result.len() != 0 {
                    return;
                }
                curr_bin.pop();
            }
        }
    }
}
