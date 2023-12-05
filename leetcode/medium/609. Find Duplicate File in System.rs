use std::collections::HashMap;
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut content_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut ans: Vec<Vec<String>> = vec![];
        // Self::reading_path(&paths[0], &mut content_map);

        for i in 0..paths.len() {
            Self::reading_path(&paths[i], &mut content_map);
        }

        for (_, path_vec) in content_map.iter() {
            if path_vec.len() > 1 {
                ans.push(path_vec.to_vec());
            }
        }

        // TEST
        // println!("{:?}", content_map);
        // TEST

        ans
    }
    pub fn reading_path(path: &String, content_map: &mut HashMap<String, Vec<String>>) {
        let mut temp: Vec<char> = path.chars().collect();
        let mut end_path_idx: usize = 0;
        while temp[end_path_idx] != ' ' {
            end_path_idx += 1;
        }
        let base_path: String = path[..end_path_idx].to_string();
        // println!("{:?}", based_path);
        // let mut files: Vec<String> = vec![];
        while end_path_idx < temp.len() {
            if temp[end_path_idx] != ' ' {
                let mut file: String = String::new();
                let mut file_content: String = String::new();
                while end_path_idx < temp.len()
                    && temp[end_path_idx] != ' '
                    && temp[end_path_idx] != '('
                {
                    file.push(temp[end_path_idx]);
                    end_path_idx += 1;
                }
                while end_path_idx < temp.len() && temp[end_path_idx] != ' ' {
                    if temp[end_path_idx] != '(' && temp[end_path_idx] != ')' {
                        file_content.push(temp[end_path_idx]);
                    }
                    end_path_idx += 1;
                }
                // println!("file:{:?}-content:{:?}", file, file_content);
                if let Some(path_vec) = content_map.get_mut(&file_content) {
                    path_vec.push(format!("{}/{}", base_path, file));
                } else {
                    content_map.insert(file_content, vec![format!("{}/{}", base_path, file)]);
                }
                // files.push(file);
                continue;
            }
            end_path_idx += 1;
        }
        // println!("{:?}", files);
    }
}
