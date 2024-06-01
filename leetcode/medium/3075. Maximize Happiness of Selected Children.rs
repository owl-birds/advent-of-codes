impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        let mut temp: Vec<i32> = happiness.to_vec();
        let mut res: i64 = 0;
        let mut count: i32 = 0;
        temp.sort_by(|a, b| a.cmp(&b));

        while temp.len() > 0 && count < k {
            let popped = temp.pop().unwrap();
            res += (if popped - count < 0 {
                0
            } else {
                popped - count
            }) as i64;
            count += 1;
        }

        res
    }
}
