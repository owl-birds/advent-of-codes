impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut x_bin: Vec<i32> = Self::dec_to_bin(x as i64);
        let mut n_1_bin: Vec<i32> = Self::dec_to_bin((n - 1) as i64);
        let mut last_num_bin: Vec<i32> = x_bin.to_vec();

        let mut i: usize = 0;
        let mut j: usize = 0;
        while j < n_1_bin.len() && i < last_num_bin.len() {
            if last_num_bin[i] == 0 {
                last_num_bin[i] = n_1_bin[j];
                j += 1;
            }
            i += 1;
        }
        while j < n_1_bin.len() {
            last_num_bin.push(n_1_bin[j]);
            j += 1;
        }

        // TLE
        // while len < n as usize {
        //     if x as i64 & res == x as i64 {
        //         len += 1;
        //         if len == n as usize {break}
        //     }
        //     res += 1;
        // }
        // TLE

        //
        // println!("x:{:?}", x_bin);
        // println!("n-1:{:?}", n_1_bin);
        // println!("::::{:?}-i:{}-j:{}", last_num_bin,i,j);
        // println!("20:{:?}", Self::dec_to_bin(20));
        // println!("221:{:?}", Self::dec_to_bin(221));
        // println!("155:{:?}", Self::dec_to_bin(155));
        // println!("15:{:?}", Self::dec_to_bin(15));
        // println!("60:{:?}", Self::dec_to_bin(60));
        // println!("47:{:?}", Self::dec_to_bin(47));
        Self::bin_to_dec(&last_num_bin, true)
    }
    pub fn dec_to_bin(dec_num: i64) -> Vec<i32> {
        let mut bin: Vec<i32> = vec![];
        let mut temp = dec_num;
        while temp > 0 {
            if temp % 2 == 0 {
                bin.push(0);
            } else {
                bin.push(1);
            }
            temp /= 2;
        }
        // bin.reverse();
        bin
    }
    pub fn bin_to_dec(bin: &Vec<i32>, is_reverse: bool) -> i64 {
        let mut res: i64 = 0;
        if is_reverse {
            for i in 0..bin.len() {
                res += (i64::pow(2, i as u32) * bin[i] as i64);
            }
        } else {
            let mut i: usize = bin.len();
            let mut pow: u32 = 0;
            while i > 0 {
                i -= 1;
                res += (i64::pow(2, pow as u32) * bin[i] as i64);
                pow += 1;
            }
        }
        res
    }
}
