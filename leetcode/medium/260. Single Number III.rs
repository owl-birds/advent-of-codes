//
// XOR (exclusive or)
// 1 ^ 1 = 0
// 0 ^ 0 = 0
// 1 ^ 0 = 1
// 0 ^ 1 = 1
// some insight
// if the same number xor-ing, it will
// always result in 0 ZERO

// bitwise problem
// [1,2,2,3,3]
// xor all the num, we will get the different num in the list
// if u only have one diff number and even count of other number
// couse uf u xor the same number it will result in 0

// but the problem is there re 2 diff num
// [1,2,2,3,3,4]

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![0, 0];
        if nums.len() == 0 {
            return res;
        }
        let mut xor_res: i32 = nums[0];

        for i in 1..nums.len() {
            xor_res ^= nums[i];
        }

        // num1 ^ num2 = xor_all !!!!!

        let mut diff_bit: i32 = 1; // 00001
                                   // << shifting operator in bits
                                   // 00001 ,,, 00010
                                   // diff_bit << 1 :: 00001 -> 00010
                                   // AND OPEARTOR IN BITS ::: &, checking every bit,
                                   // println!("{}", 8 & xor_res);
                                   // println!("{}", 2 & xor_res);
                                   // https://doc.rust-lang.org/book/appendix-02-operators.html

        while diff_bit & xor_res == 0 {
            diff_bit = diff_bit << 1;
            // println!("{:?}", Self::dec_to_bin(diff_bit));
        }

        for i in 0..nums.len() {
            // println!("{}-{:?}-{}:{}", nums[i] & diff_bit, Self::dec_to_bin(nums[i]), diff_bit, nums[i]);
            if nums[i] & diff_bit != 0 {
                res[0] ^= nums[i];
            } else {
                res[1] ^= nums[i];
            }
        }

        // // lazy way to find the diff bits, but not working with negative num
        // let mut xor_bin = Self::dec_to_bin(xor_res);
        // let mut first_1: usize = 0;
        // let mut idx: usize = xor_bin.len();
        // // so we re finding the ix of first bit 1 in the bin of xor all
        // while idx > 0 {
        //     idx -= 1;
        //     if xor_bin[idx] == 1 {
        //         first_1 = idx;
        //         break;
        //     }
        // }
        // // lazy way to find the diff bits, but not working with negative num

        // for i in 0..nums.len() {
        //     // lazy way to find the diff bits, but not working with negative num
        //     let mut curr_bin = Self::dec_to_bin(nums[i]);
        //     curr_bin.reverse();
        //     if curr_bin.len() < xor_bin.len() {
        //         let mut c = 0;
        //         let len = curr_bin.len();
        //         while c < xor_bin.len() - len {
        //             curr_bin.push(0);
        //             c += 1;
        //         }
        //     }
        //     curr_bin.reverse();
        //     // println!("{:?}-{}-{}", curr_bin,first_1,curr_bin.len());
        //     if curr_bin.len() > 0 && curr_bin[first_1] == 1 {
        //         res[0] ^= nums[i];
        //     } else {
        //         res[1] ^= nums[i];
        //     }
        //     // lazy way to find the diff bits, but not working with negative num
        // }

        // println!("-----------\n{}", xor_res);
        // println!("{:?}-{}", Self::dec_to_bin(5), 5);
        // println!("{:?}-{}", Self::dec_to_bin(3), 3);
        // println!("{:?}-{}", Self::dec_to_bin(4), 4);
        // println!("xor:{:?}-{}", Self::dec_to_bin(xor_res), xor_res);
        // println!("{}", 6 & -6);
        // println!("{}", 1 & 1);
        // println!("{}", 10 & 1);

        // // a little bit cheating, using sort, and of course worst then set solution
        // let mut temp = nums.to_vec();
        // temp.sort_by(|a, b| a.cmp(&b));
        // println!("{:?}", temp);
        // let mut idx: usize = 0;
        // while idx < temp.len() {
        //     // println!("{}", idx);
        //     if idx == temp.len()-1 {
        //         res.push(temp[idx]);
        //         idx += 1;
        //         break;
        //     }
        //     if temp[idx] ^ temp[idx+1] != 0 {
        //         res.push(temp[idx]);
        //         if idx+1 < temp.len()-1 && temp[idx+1] != temp[idx+2] || (idx+1) == temp.len()-1 {
        //             res.push(temp[idx+1]);
        //             idx += 1;
        //         }
        //         idx += 1;
        //         continue;
        //     }
        //     idx += 2;
        // }
        // // a little bit cheating, using sort

        res
    }
    pub fn dec_to_bin(n: i32) -> Vec<i32> {
        // only positive integer
        if n == 0 {
            return vec![0];
        }
        let mut bin: Vec<i32> = vec![];
        let mut temp: i32 = n;
        while temp > 0 {
            if temp % 2 != 0 {
                bin.push(1)
            } else {
                bin.push(0)
            }
            temp /= 2;
        }
        // let mut c = 0;
        // while c < 16-bin.len() {
        //     bin.push(0);
        //     c += 1;
        // } // memory limit exceeded
        bin.reverse();
        bin
    }
}
