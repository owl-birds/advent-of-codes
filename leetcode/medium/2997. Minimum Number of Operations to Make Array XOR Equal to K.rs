// XOR :
// 1 1 = 0
// 0 0 = 0
// 1 0 = 1
// 0 1 = 1

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut res: i32 = 0;
        let mut bins: Vec<Vec<i32>> = vec![];
        let mut bin_k: Vec<i32> = Self::dec_bin(k);
        for i in 0..nums.len() {
            bins.push(Self::dec_bin(nums[i]));
        }
        let mut xor_all: Vec<i32> = Self::xor_all(&bins);

        // println!("xor_all:{:?}", xor_all);
        // println!("bin_k:{:?}", bin_k);
        // println!("{:?}", bins);
        // println!("{:?}", Self::xor_all(&bins));
        // println!("{:?}", Self::dec_bin(k));

        let mut is_xor_longer: bool = if xor_all.len() > bin_k.len() {
            true
        } else {
            false
        };
        if is_xor_longer {
            for i in 0..xor_all.len() - bin_k.len() {
                bin_k.push(0);
            }
        } else {
            for i in 0..bin_k.len() - xor_all.len() {
                xor_all.push(0);
            }
        }
        // println!("-----");
        // println!("xor_all:{:?}", xor_all);
        // println!("bin_k:{:?}", bin_k);

        for i in 0..xor_all.len() {
            if xor_all[i] != bin_k[i] {
                res += 1;
            }
        }

        res
    }
    pub fn xor_all(bins: &Vec<Vec<i32>>) -> Vec<i32> {
        if bins.len() <= 0 {
            return vec![];
        }
        let mut res: Vec<i32> = vec![];
        if bins.len() == 1 {
            return bins[0].to_vec();
        }
        let mut is_first_longer: bool = if bins[0].len() > bins[1].len() {
            true
        } else {
            false
        };
        if is_first_longer {
            for i in 0..bins[1].len() {
                if bins[0][i] != bins[1][i] {
                    res.push(1);
                } else {
                    res.push(0);
                }
            }
            for i in bins[1].len()..bins[0].len() {
                if bins[0][i] != 0 {
                    res.push(1);
                } else {
                    res.push(0);
                }
            }
        } else {
            for i in 0..bins[0].len() {
                if bins[0][i] != bins[1][i] {
                    res.push(1);
                } else {
                    res.push(0);
                }
            }
            for i in bins[0].len()..bins[1].len() {
                if bins[1][i] != 0 {
                    res.push(1);
                } else {
                    res.push(0);
                }
            }
        }

        // println!("----");
        // println!("{:?}", res);
        // println!("----");

        for i in 2..bins.len() {
            let mut is_first_longer: bool = if res.len() > bins[i].len() {
                true
            } else {
                false
            };
            if is_first_longer {
                for j in 0..bins[i].len() {
                    if res[j] != bins[i][j] {
                        res[j] = 1;
                    } else {
                        res[j] = 0;
                    }
                }
                for j in bins[i].len()..res.len() {
                    if res[j] != 0 {
                        res[j] = 1;
                    } else {
                        res[j] = 0;
                    }
                }
            } else {
                for j in 0..res.len() {
                    if res[j] != bins[i][j] {
                        res[j] = 1;
                    } else {
                        res[j] = 0;
                    }
                }
                for j in res.len()..bins[i].len() {
                    if bins[i][j] != 0 {
                        res.push(1);
                    } else {
                        res.push(0);
                    }
                }
            }
            // println!("-------\n{:?}\n-----", res);
        }

        res
    }
    pub fn dec_bin(n: i32) -> Vec<i32> {
        if n == 0 {
            return vec![0];
        }
        let mut bin: Vec<i32> = vec![];
        let mut temp: i32 = n;

        while temp > 0 {
            if temp % 2 != 0 {
                bin.push(1);
            } else {
                bin.push(0);
            }
            temp /= 2;
        }
        // bin.reverse();
        bin
    }
}
