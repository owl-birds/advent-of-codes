impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        if pref.len() == 0 {
            return vec![];
        }
        let mut result: Vec<i32> = vec![pref[0]];

        for i in 1..pref.len() {
            result.push(pref[i] ^ pref[i - 1]);
        }

        result
    }
}
