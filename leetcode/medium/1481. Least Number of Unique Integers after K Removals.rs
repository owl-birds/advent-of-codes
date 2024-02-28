use std::collections::HashMap;

impl Solution {
    pub fn find_least_num_of_unique_ints(arr: Vec<i32>, k: i32) -> i32 {
        let mut num_freq: HashMap<i32, i32> = HashMap::new();

        for i in 0..arr.len() {
            if let Some(freq) = num_freq.get_mut(&arr[i]) {
                *freq += 1;

                continue;
            }

            num_freq.insert(arr[i], 1);
        }

        let mut temp: Vec<Vec<i32>> = vec![];

        for (num, freq) in num_freq.iter() {
            temp.push(vec![*num, *freq]);
        }

        temp.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut temp_k: i32 = k;

        let mut deleted: i32 = 0;

        let mut idx: usize = 0;

        while temp_k > 0 && idx < temp.len() {
            if temp[idx][1] <= temp_k {
                temp_k -= temp[idx][1];

                deleted += 1;
            } else {
                temp_k = 0;
            }

            idx += 1;
        }

        temp.len() as i32 - deleted
    }
}
