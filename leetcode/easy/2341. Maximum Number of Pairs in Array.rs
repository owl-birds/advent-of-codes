use std::collections::HashMap;
impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        let mut ans: Vec<i32> = vec![0, 0];

        for num in nums {
            if let Some(count) = count_map.get_mut(&num) {
                *count += 1;
                continue;
            }
            count_map.insert(num, 1);
        }
        // println!("{:?}", count_map);
        for (num, count) in count_map.iter() {
            ans[0] += (count / 2);
            ans[1] += (count % 2);
        }

        ans
    }
}
