use std::collections::HashSet;
impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let mut count: i32 = 0;
        let mut cache: HashSet<i32> = HashSet::new();
        for num in left..right + 1 {
            // println!("bits:{}", Self::count_bits(num));
            let bits = Self::count_bits(num);
            if cache.contains(&bits) {
                count += 1;
                continue;
            }
            if Self::is_prime(bits) {
                // println!("bits:{}", bits);
                cache.insert(bits);
                count += 1;
            }
        }

        count
    }
    pub fn count_bits(num: i32) -> i32 {
        let mut temp = num;
        let mut count = 0;

        while temp > 0 {
            if temp % 2 != 0 {
                count += 1
            }
            temp /= 2;
        }

        count
    }
    pub fn is_prime(num: i32) -> bool {
        if num <= 1 {
            return false;
        }

        for i in 2..num / 2 + 1 {
            if num % i == 0 {
                return false;
            }
        }

        true
    }
}
