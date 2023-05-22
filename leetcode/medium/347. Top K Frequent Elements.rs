use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        let mut frequencies: Vec<i32> = vec![];
        for i in 0..nums.len(){
            if !num_freq.contains_key(&nums[i]){
                num_freq.insert(nums[i], 1);;
                continue;
            }
            if let Some(freq) = num_freq.get_mut(&nums[i]){
                *freq += 1;
            }
        }

        // new map
        let mut freq_nums: HashMap<i32, Vec<i32>> = HashMap::new();

        for (key, val) in num_freq.iter(){
            frequencies.push(*val);
            if !freq_nums.contains_key(val){
                freq_nums.insert(*val, vec![*key]);
                continue;
            }
            if let Some(nums) = freq_nums.get_mut(val){
                nums.push(*key);
            }
        }
        frequencies.sort_by(|a, b| b.cmp(a));
        
        let mut result_set: HashSet<i32> = HashSet::new();
        for i in 0..k as usize{
            if !result_set.contains(&frequencies[i]){
                result_set.insert(frequencies[i]);
            }
            
        }
        for freq in result_set.iter(){
            if let Some(nums) = freq_nums.get_mut(freq){
                for j in 0..nums.len(){
                    result.push(nums[j]);
                }
            }
        }

        // println!("nums {:?}", nums);
        // println!("num_freq {:?}", num_freq);
        // println!("freq_nums {:?}", freq_nums);
        // println!("freq {:?}", frequencies);
        // println!("result {:?}", result);
    
    
        result       
    }
}