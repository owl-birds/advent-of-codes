impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        let mut result: i32 = -1;
        let mut pref_sum: Vec<i32> = vec![];
        let mut curr_sum: i32 = 0;
        for num in 1..n + 1 {
            curr_sum += num;
            pref_sum.push(curr_sum);
        }
        // println!("{:?}", pref_sum);
        for i in 1..pref_sum.len() {
            if pref_sum[i - 1] == (pref_sum[pref_sum.len() - 1] - pref_sum[i]) {
                result = i as i32 + 1;
            }
        }
        result
    }
}
