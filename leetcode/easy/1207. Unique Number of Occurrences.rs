use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        let mut freq_sets: HashSet<i32> = HashSet::new();

        for i in 0..arr.len() {
            if let Some(freq) = num_freq.get_mut(&arr[i]) {
                *freq += 1;
            } else {
                num_freq.insert(arr[i], 1);
            }
        }
        for (_, freq) in num_freq.iter() {
            freq_sets.insert(*freq);
        }

        freq_sets.len() == num_freq.len()
    }
}
