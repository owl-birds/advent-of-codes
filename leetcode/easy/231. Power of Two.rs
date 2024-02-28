impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        Self::count_one_from_bin(n) == 1
    }
    pub fn count_one_from_bin(num: i32) -> i32 {
        let mut temp: i32 = num;
        let mut count: i32 = 0;

        while temp > 0 {
            if temp % 2 != 0 {
                count += 1
            }
            temp /= 2;
        }

        count
    }
}
