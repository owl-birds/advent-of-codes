impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        let mut start_bin: Vec<u32> = Self::dec_to_bin(start);
        let mut goal_bin: Vec<u32> = Self::dec_to_bin(goal);
        if start_bin.len() > goal_bin.len() {
            for i in 0..(start_bin.len() - goal_bin.len()) {
                goal_bin.push(0);
            }
        } else if start_bin.len() < goal_bin.len() {
            for i in 0..(goal_bin.len() - start_bin.len()) {
                start_bin.push(0);
            }
        }
        let mut count: i32 = 0;

        for i in 0..goal_bin.len() {
            if goal_bin[i] == 1 && start_bin[i] == 0 {
                count += 1
            }
            if goal_bin[i] == 0 && start_bin[i] == 1 {
                count += 1
            }
        }

        count
    }
    pub fn dec_to_bin(num: i32) -> Vec<u32> {
        let mut temp_num = num;
        let mut bin_vec: Vec<u32> = vec![];

        while temp_num > 0 {
            if temp_num % 2 == 0 {
                bin_vec.push(0)
            } else {
                bin_vec.push(1)
            }
            temp_num /= 2;
        }

        bin_vec
    }
}
