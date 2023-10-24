impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n == 1 {
            return true;
        }
        if n < 0 || n % 4 != 0 {
            return false;
        }
        // // the APE way
        // let mut temp: i32 = n;
        // while (temp > 0) {
        //     temp -= 4;
        // }
        // if temp < 0 {return false}
        // // the APE way

        // the MODERN APE way
        let temp = Self::checking_bin(n);
        if temp == -1 {
            return false;
        }
        if temp % 2 == 0 {
            return false;
        }
        // println!("64: {}", Self::checking_bin(64));
        // println!("256: {}", Self::checking_bin(256));
        // the MODERN APE way

        true
    }
    pub fn checking_bin(num: i32) -> i32 {
        let mut idx: i32 = -1;
        let mut temp = if num < 0 { num * -1 } else { num };
        let mut temp_idx: i32 = 1;
        while temp > 0 {
            if temp % 2 != 0 {
                if idx != -1 {
                    return -1;
                }
                idx = temp_idx;
            }
            temp /= 2;
            temp_idx += 1;
        }
        if num < 0 && idx % 2 != 0 {
            return -1;
        }
        idx
    }
}
