impl Solution {
    pub fn max_bottles_drunk(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut max_drank: i32 = 0;
        let mut full: i32 = num_bottles;
        let mut empty: i32 = 0;
        let mut exchange: i32 = num_exchange;

        while full > 0 || empty >= exchange {
            if empty < exchange {
                max_drank += full;
                empty += full;
                full = 0;
                continue;
            }
            empty -= exchange;
            full += 1;
            exchange += 1;
        }

        max_drank
    }
}
