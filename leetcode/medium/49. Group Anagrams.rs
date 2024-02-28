use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let mut temp_strs: Vec<Vec<String>> = vec![];
        for s in &strs {
            let mut temp: Vec<char> = s.chars().collect();
            temp.sort_by(|a, b| a.cmp((&b)));
            temp_strs.push(vec![Self::vec_to_string(&temp), s.to_string()]);
        }
        // println!("{:?}", temp_strs);
        let mut group_map: HashMap<String, Vec<String>> = HashMap::new();
        for i in 0..temp_strs.len() {
            if let Some(str_vec) = group_map.get_mut(&temp_strs[i][0]) {
                str_vec.push(temp_strs[i][1].to_string());
                continue;
            }
            group_map.insert(
                temp_strs[i][0].to_string(),
                vec![temp_strs[i][1].to_string()],
            );
        }
        // println!("{:?}", group_map);
        for (_, str_group) in group_map.iter() {
            result.push(str_group.to_vec());
        }
        result
    }
    pub fn vec_to_string(char_vec: &Vec<char>) -> String {
        let mut str_result: String = String::new();

        for c in char_vec {
            str_result.push(*c);
        }

        str_result
    }
}
