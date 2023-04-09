use std::collections::HashMap;
impl Solution {
    pub fn partition_string(s: String) -> i32 {
        // // use map
        // let test = String::from("TEST");
        // let test_2: String = format!("{} HELLO", test);
        // println!("{test_2}");
        
        
        let mut substring_map: HashMap<char, bool> = HashMap::new();
        let mut count_partition: i32 = 0;

        for c in s.chars(){
            // println!("{}", c);
            if !substring_map.contains_key(&c){
                substring_map.insert(c, true);
                continue;
            }
            // println!("{}", c);
            substring_map.clear();
            // substring_map = HashMap::new();
            substring_map.insert(c, true);
            count_partition += 1;
        }
        // println!("{:?}", substring_map);
        // println!("{}", substring_map);
        count_partition + 1
    }
}