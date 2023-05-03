use std::collections::HashMap;

impl Solution {
    pub fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) ->     Vec<Vec<i32>> {
        let mut map_1: HashMap<i32, bool> = HashMap::new();        
        let mut map_2: HashMap<i32, bool> = HashMap::new();        
        let mut result: Vec<Vec<i32>> = vec![vec![],vec![]];
        for num in &nums1{
            if !map_1.contains_key(num){
                map_1.insert(*num, true);
            }
        }
        for num in &nums2{
            if !map_2.contains_key(num){
                map_2.insert(*num, true);
            }
        }
        // checking
        for (num, _) in &map_1{
            if map_2.contains_key(num){
                continue;
            }
            result[0].push(*num);
        }
        for (num, _) in &map_2{
            if map_1.contains_key(num){
                continue;
            }
            result[1].push(*num);
        }
        result
    }
}