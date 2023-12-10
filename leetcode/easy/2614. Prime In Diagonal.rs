use std::collections::HashSet;
impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut max_prime: i32 = i32::MIN;
        let mut chache_prime: HashSet<i32> = HashSet::new();

        let mut r: usize = 0;
        let mut c: usize = 0;

        while r < nums.len() && c < nums[0].len() {
            if chache_prime.contains(&nums[r][c]) {
                c += 1;
                r += 1;
                continue;
            }
            if Self::is_prime(nums[r][c]) && nums[r][c] > max_prime {
                chache_prime.insert(nums[r][c]);
                max_prime = nums[r][c];
            }
            c += 1;
            r += 1;
        }

        let mut r: usize = 0;
        let mut c: usize = nums[0].len();

        while r < nums.len() && c > 0 {
            c -= 1;
            if chache_prime.contains(&nums[r][c]) {
                r += 1;
                continue;
            }
            if Self::is_prime(nums[r][c]) && nums[r][c] > max_prime {
                chache_prime.insert(nums[r][c]);
                max_prime = nums[r][c];
            }
            r += 1;
        }

        // // TEST
        // println!("11:{}", Self::is_prime(11));
        // println!("6:{}", Self::is_prime(6));
        // println!("13:{}", Self::is_prime(13));
        // println!("7:{}", Self::is_prime(7));
        // println!("9:{}", Self::is_prime(9));
        // println!("985:{}", Self::is_prime(985));
        // // TEST

        if max_prime == i32::MIN {
            return 0;
        }
        max_prime
    }
    pub fn is_prime(n: i32) -> bool {
        if n == 1 {
            return false;
        }
        for div in 2..((n as f64).sqrt() as i32) + 1 {
            if n % div == 0 {
                return false;
            }
        }
        true
    }
}
