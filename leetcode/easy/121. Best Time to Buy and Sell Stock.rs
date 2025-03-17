impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit: i32 = 0;
        let mut bought: usize = 0;
        let mut sold: usize = 1;

        while sold < prices.len() && bought < sold {
            if prices[bought] > prices[sold] {
                bought = sold;
            }
            let temp = prices[sold] - prices[bought];
            profit = if temp > profit { temp } else { profit };
            sold += 1;
        }

        profit
    }
}
