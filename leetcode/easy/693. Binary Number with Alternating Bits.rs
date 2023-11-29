impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut temp: i32 = n;
        let mut prev_bit: i32 = if temp % 2 == 0 { 0 } else { 1 };
        temp /= 2;
        while temp > 0 {
            let curr_bit: i32 = if temp % 2 == 0 { 0 } else { 1 };
            if curr_bit == prev_bit {
                return false;
            }
            prev_bit = curr_bit;
            temp /= 2;
        }

        true
    }
}
