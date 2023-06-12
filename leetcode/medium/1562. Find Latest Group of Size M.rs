impl Solution {
    pub fn find_latest_step(arr: Vec<i32>, m: i32) -> i32 {
        // TLE SOLUTION
        let mut at_process: i32 = -1;
        let mut bin_digits: Vec<i32> = Vec::new();
        for i in 0..arr.len() {
            bin_digits.push(0);
        }
        // println!("{:?}", bin_digits);

        for i in 0..arr.len() {
            bin_digits[(arr[i] - 1) as usize] = 1;
            if ((i + 1) as i32) < m {
                continue;
            }
            if Self::is_exist(&bin_digits, m) {
                at_process = (i + 1) as i32;
            }
            // println!("{:?}", bin_digits);
        }

        // TESt
        // println!("{}", Self::is_exist(&vec![0,1,1,0,1,1,0,1,1,0,1,0,1,1,1,1,1,0], 5));
        // println!("{}", Self::is_exist(&vec![0,1,1,0,1,1,0,1,1,0,1,0,1,1,1,1,1,0], 5));
        // println!("{}", Self::is_exist(&vec![1, 1, 1, 0, 1], 1));
        // TESt

        at_process
        // TLE SOLUTION
    }
    // TLE SOLUTION
    pub fn is_exist(bin_digits: &Vec<i32>, m: i32) -> bool {
        let mut is_one: bool = false;
        let mut idx: usize = 0;

        while idx < bin_digits.len() {
            if bin_digits[idx] == 1 {
                let mut count: i32 = 0;
                // loop till found zero
                while idx < bin_digits.len() && bin_digits[idx] == 1 {
                    count += 1;
                    idx += 1;
                }
                if count == m {
                    return true;
                }
                continue;
            }
            idx += 1;
        }
        false
    }
    // TLE SOLUTION
}
