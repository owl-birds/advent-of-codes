use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut degree: i32 = 0;
        let mut degree_nums: HashSet<i32> = HashSet::new();
        for i in 0..nums.len() {
            if let Some(count) = map.get_mut(&nums[i]) {
                *count += 1;
                if *count > degree {
                    degree = *count;
                }
            } else {
                map.insert(nums[i], 1);
                if degree < 1 {
                    degree = 1;
                }
            }
        }
        for (num, count) in map.iter() {
            if *count == degree {
                degree_nums.insert(*num);
            }
        }
        // println!("degree:{}", degree);
        // println!("degree_num:{:?}", degree_nums);
        // println!("map_freq:{:?}", map);

        let mut min_length: i32 = nums.len() as i32;
        for num in degree_nums.iter() {
            let mut start_idx: usize = 0;
            for i in 0..nums.len() {
                if nums[i] == *num {
                    start_idx = i;
                    break;
                }
            }
            let mut end_idx: usize = start_idx + 1;
            for i in 0..nums.len() {
                if nums[i] == *num {
                    end_idx = i;
                }
            }
            let mut temp_length: i32 = (end_idx - start_idx + 1) as i32;
            if temp_length < min_length {
                min_length = temp_length;
            }
        }

        min_length
    }
}
