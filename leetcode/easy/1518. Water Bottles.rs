impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut res: i32 = num_bottles;
        let mut empty_bot: i32 = num_bottles;

        while empty_bot >= num_exchange {
            res += (empty_bot / num_exchange);
            let temp = empty_bot;
            empty_bot = empty_bot % num_exchange;
            empty_bot += (temp / num_exchange);
        }

        res
    }
}
