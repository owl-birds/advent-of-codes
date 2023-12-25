use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut dest: String = String::new();

        // dumb solution
        let mut idx_set: HashSet<usize> = HashSet::new();
        // let mut curr_path: Vec<Vec<String>> = vec![];
        let mut curr_path: VecDeque<Vec<String>> = VecDeque::new();
        while idx_set.len() != paths.len() {
            if curr_path.len() == 0 {
                curr_path.push_back(paths[0].to_vec());
                idx_set.insert(0);
                continue;
            }
            // finding prev_dest
            for i in 0..paths.len() {
                if idx_set.contains(&i) {
                    continue;
                }
                if curr_path[0][0] == paths[i][1] {
                    curr_path.push_front(paths[i].to_vec());
                    idx_set.insert(i);
                    break;
                }
            }
            // finding next_dest
            for i in 0..paths.len() {
                if idx_set.contains(&i) {
                    continue;
                }
                if curr_path[curr_path.len() - 1][1] == paths[i][0] {
                    curr_path.push_back(paths[i].to_vec());
                    idx_set.insert(i);
                    break;
                }
            }
            // break;
        }
        // println!("{:?}", curr_path);
        // if let Some(last_stop) = curr_path.get(curr_path.len()-1) {
        //     // println!("{:?}", String::from(last_stop));
        //     for c in last_stop[1].chars() {
        //         dest.push(c);
        //     }
        // }
        let last_stop: Vec<String> = curr_path.pop_back().unwrap();
        dest = last_stop[1].to_string();
        dest
    }
}
