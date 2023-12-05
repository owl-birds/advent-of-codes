impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut money: i32 = 0;
        let mut week: i32 = 0;
        // Sn = n/2 [2a + (n - 1) d]
        while week < (n / 7) {
            money += (7) * (week + 1 + 3);
            week += 1;
        }
        if n % 7 != 0 {
            money += (n % 7) * ((2 * (week + 1)) + ((n % 7) - 1)) / 2;
        }

        money
    }
}
