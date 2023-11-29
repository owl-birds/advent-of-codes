impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let mut count: i32 = 0;
        let mut temp: u32 = n;
        while temp > 0 {
            if temp % 2 != 0 {
                count += 1
            }
            temp /= 2;
        }

        count
    }
}
