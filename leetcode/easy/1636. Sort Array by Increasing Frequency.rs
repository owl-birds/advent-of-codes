use std::collections::HashMap;
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut freq_map: HashMap<i32, i32> = HashMap::new();
        for num in nums {
            if let Some(freq) = freq_map.get_mut(&num) {
                *freq += 1;
                continue;
            }
            freq_map.insert(num, 1);
        }
        let mut freq_vec: Vec<Vec<i32>> = vec![];
        for (num, freq) in freq_map.iter() {
            freq_vec.push(vec![*num, *freq]);
        }
        freq_vec.sort_by(|a, b| {
            if a[1] == b[1] {
                b[0].cmp(&a[0])
            } else {
                a[1].cmp(&b[1])
            }
        });

        // inspiration : https://users.rust-lang.org/t/solved-advanced-sort-on-vector-of-slice/13477/7
        // v_dirs.sort_by(|a, b| {
        //     if a.1.is_file() && b.1.is_dir() { Ordering::Equal }
        //     else if a.1.is_dir() && b.1.is_file() { Ordering::Greater }
        //     else { a.cmp(b) }
        // });

        // println!("{:?}", freq_vec);
        let mut result: Vec<i32> = vec![];
        for num_freq in freq_vec {
            for i in 0..num_freq[1] {
                result.push(num_freq[0]);
            }
        }

        result
    }
}
