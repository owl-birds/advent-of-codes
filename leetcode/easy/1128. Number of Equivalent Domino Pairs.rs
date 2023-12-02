use std::collections::HashMap;
impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut count: i32 = 0;
        let mut history: HashMap<String, i32> = HashMap::new();
        for i in 0..dominoes.len() {
            let mut dom: String = String::new();
            dom.push_str(&format!("{}{}", dominoes[i][0], dominoes[i][1])[..]);
            let mut dom_rev: String = String::new();
            dom_rev.push_str(&format!("{}{}", dominoes[i][1], dominoes[i][0])[..]);
            // println!("{:?} -- {:?}", dom, dom_rev);
            let mut temp_freq: i32 = -1;
            if let Some(freq) = history.get_mut(&dom) {
                temp_freq = *freq;
                *freq += 1;
            } else {
                // temp_freq = 1;
                history.insert(dom, 1);
            }
            if dominoes[i][0] != dominoes[i][1] {
                if let Some(freq) = history.get_mut(&dom_rev) {
                    temp_freq = *freq;
                    *freq += 1;
                } else {
                    // temp_freq = 1;
                    history.insert(dom_rev, 1);
                }
            }
            // println!("{:?}", history);
            // println!("{}", temp_freq);
            if temp_freq != -1 {
                count += (temp_freq);
            }
        }

        count
    }
}
