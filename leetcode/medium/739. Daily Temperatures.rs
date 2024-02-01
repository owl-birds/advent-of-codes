use std::collections::VecDeque;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        let mut mon_stack: VecDeque<usize> = VecDeque::new();
        let mut r_idx: usize = temperatures.len();
        let mut idx: usize = temperatures.len();
        while idx > 0 {
            idx -= 1;
            // println!("{:?}", mon_stack);
            while !mon_stack.back().is_none()
                && temperatures[*mon_stack.back().unwrap()] <= temperatures[idx]
            {
                mon_stack.pop_back();
            }
            if (!mon_stack.is_empty()) {
                result[idx] = (*mon_stack.back().unwrap() - idx) as i32;
            }
            mon_stack.push_back(idx);
        }

        result
    }
}
