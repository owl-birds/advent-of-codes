impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut count: i32 = 0;
        let mut a_bin: Vec<u32> = Self::dec_to_bin(a); // reverse bin num
        let mut b_bin: Vec<u32> = Self::dec_to_bin(b); // reverse bin num
        let mut c_bin: Vec<u32> = Self::dec_to_bin(c); // reverse bin num
        println!("a:{:?}\nb:{:?}\nc:{:?}", a_bin, b_bin, c_bin);

        let mut max_length = a_bin.len();
        if max_length < b_bin.len() {
            max_length = b_bin.len();
        }
        if max_length < c_bin.len() {
            max_length = c_bin.len();
        }
        println!("{}", max_length);
        if a_bin.len() < max_length {
            for i in 0..(max_length - a_bin.len()) {
                a_bin.push(0);
            }
        }
        if b_bin.len() < max_length {
            for i in 0..(max_length - b_bin.len()) {
                b_bin.push(0);
            }
        }
        if c_bin.len() < max_length {
            for i in 0..(max_length - c_bin.len()) {
                c_bin.push(0);
            }
        }
        println!("a:{:?}\nb:{:?}\nc:{:?}", a_bin, b_bin, c_bin);

        // COUNT
        for i in 0..c_bin.len() {
            let c_bit = c_bin[i];
            let a_bit = a_bin[i];
            let b_bit = b_bin[i];

            if c_bit == 1 {
                if a_bit == 0 && b_bit == 0 {
                    count += 1;
                }
            } else {
                if a_bit == 1 && b_bit == 1 {
                    count += 2;
                } else if a_bit == 1 || b_bit == 1 {
                    count += 1;
                }
            }
        }
        // COUNT

        count
    }
    pub fn dec_to_bin(num: i32) -> Vec<u32> {
        let mut temp_num = num;
        let mut bin_vec: Vec<u32> = vec![];

        while temp_num > 0 {
            if temp_num % 2 == 0 {
                bin_vec.push(0)
            } else {
                bin_vec.push(1)
            }
            temp_num /= 2;
        }

        bin_vec
    }
}
