use std::collections::HashMap;
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut alps: [char; 26] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let mut alps_map: HashMap<char, i32> = HashMap::new();
        for i in 0..alps.len() {
            alps_map.insert(alps[i], i as i32 + 1);
        }
        // println!("{:?}", alps_map);

        let mut result: i32 = 0;
        let mut temp_title: Vec<char> = column_title.chars().collect();
        let mut idx: usize = temp_title.len();
        // let mut base: i32 = 26; ---> base 26
        let mut pow: u32 = 0;
        while idx >= 1 {
            idx -= 1;
            if let Some(val) = alps_map.get(&temp_title[idx]) {
                result += *val * i32::pow(26, pow);
            }
            pow += 1;
        }

        result
    }
}
