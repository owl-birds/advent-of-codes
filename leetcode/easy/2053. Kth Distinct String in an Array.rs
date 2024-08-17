use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut new_map: HashMap<String, i32> = HashMap::new();
        let mut temp: Vec<String> = vec![];
        let mut temp_set: HashSet<String> = HashSet::new();
        // let mut res: String = String::from("");

        for i in 0..arr.len() {
            if let Some(count) = new_map.get_mut(&arr[i]) {
                *count += 1;
            } else {
                new_map.insert(arr[i].to_string(), 1);
            }
        }
        // println!("{:?}", new_map);
        for i in 0..arr.len() {
            if let Some(count) = new_map.get(&arr[i]) {
                if *count == 1 && !temp_set.contains(&arr[i]) {
                    temp.push(arr[i].to_string());
                    temp_set.insert(arr[i].to_string());
                }
            }
        }
        if temp.len() < k as usize {
            return "".to_string();
        }
        temp[(k - 1) as usize].to_string()
    }
}
