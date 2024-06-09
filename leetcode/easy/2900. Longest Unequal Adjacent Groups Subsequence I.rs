impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        if words.len() == 1 {
            res.push(words[0].clone());
            return res;
        }
        let mut temp: Vec<usize> = vec![];
        for i in 1..groups.len() {
            if groups[i] != groups[i - 1] {
                if temp.len() == 0 {
                    temp.push(i - 1);
                }
                temp.push(i);
            }
        }
        if temp.len() == 0 {
            return vec![words[0].clone()];
        }
        // println!("{:?}", temp);
        for i in 0..temp.len() {
            res.push(words[temp[i]].clone());
        }
        res
    }
}
