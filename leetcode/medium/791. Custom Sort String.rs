use std::cmp::Ordering;
use std::collections::HashMap;
impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut char_ord: HashMap<char, i32> = HashMap::new();
        let mut temp_s: Vec<char> = s.chars().collect();
        let mut ord: i32 = 1;
        for c in order.chars() {
            char_ord.insert(c, ord);
            ord += 1;
        }
        temp_s.sort_by(|a, b| {
            if let Some(ord_a) = char_ord.get(a) {
                if let Some(ord_b) = char_ord.get(b) {
                    return ord_a.cmp(ord_b);
                }
                return Ordering::Greater;
            }
            if char_ord.contains_key(b) {
                return Ordering::Less; // the char that doest exist in the s String will always be less
            }
            Ordering::Less
        });
        let mut result: String = String::new();
        for c in &temp_s {
            result.push(*c);
        }

        // TEST
        // println!("{:?}", char_ord);
        // println!("{:?}", temp_s);
        // TEST
        result
    }
}
