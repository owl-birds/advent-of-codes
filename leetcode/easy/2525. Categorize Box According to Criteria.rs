use std::collections::HashSet;

impl Solution {
    pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
        let dimensions: Vec<i32> = vec![length, width, height];
        let mut category: HashSet<String> = HashSet::new();
        for i in 0..dimensions.len(){
            if dimensions[i] >= (10 as i32).pow(4){
                category.insert("Bulky".to_string());
                break;
            }
        }
        if (length as i64 * width as i64 * height as i64) >= (10 as i64).pow(9){
            category.insert("Bulky".to_string());
        }
        if mass >= 100{
            category.insert("Heavy".to_string());
        }
        if category.len() == 2{return "Both".to_string()}
        for c in category.iter(){
            return c.to_string();
        }
        "Neither".to_string()
    }
}