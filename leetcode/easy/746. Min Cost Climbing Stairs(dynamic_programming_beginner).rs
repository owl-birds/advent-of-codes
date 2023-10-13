impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = vec![0; cost.len() + 1];
        dp[0] = 0;
        dp[1] = cost[0];
        for i in 2..dp.len() {
            let f_n = cost[i - 1]
                + if dp[i - 1] > dp[i - 2] {
                    dp[i - 2]
                } else {
                    dp[i - 1]
                };
            dp[i] = f_n;
        }
        // println!("{:?}", dp);

        if dp[dp.len() - 1] > dp[dp.len() - 2] {
            return dp[dp.len() - 2];
        }
        dp[dp.len() - 1]
    }
}
