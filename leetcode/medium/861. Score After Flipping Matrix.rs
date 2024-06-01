impl Solution {
    pub fn matrix_score(grid: Vec<Vec<i32>>) -> i32 {
        let mut res: i32 = 0;
        let mut temp: Vec<Vec<i32>> = vec![];
        for i in 0..grid.len() {
            temp.push(grid[i].to_vec());
        }
        // println!("{}", Self::bin_dec(&grid[0]));
        // println!("{}", Self::bin_dec(&vec![0]));
        // println!("{}", Self::bin_dec(&vec![1]));

        // row rule, if zero first flip, if not dont flip
        for i in 0..temp.len() {
            if temp[i][0] == 0 {
                for j in 0..temp[i].len() {
                    if temp[i][j] == 0 {
                        temp[i][j] = 1;
                    } else {
                        temp[i][j] = 0;
                    }
                }
            }
        }
        // println!("{:?}", temp);

        // col rule, compare zeros and ones, if freq of zero is bigger than freq of one flip it, otherwise dont flip
        for i in 0..temp[0].len() {
            let mut zero_count: i32 = 0;
            let mut one_count: i32 = 0;
            for j in 0..temp.len() {
                if temp[j][i] == 0 {
                    zero_count += 1;
                } else {
                    one_count += 1;
                }
            }
            if zero_count > one_count {
                for j in 0..temp.len() {
                    if temp[j][i] == 0 {
                        temp[j][i] = 1;
                    } else {
                        temp[j][i] = 0;
                    }
                }
            }
        }
        // println!("{:?}", temp);
        for i in 0..temp.len() {
            res += Self::bin_dec(&temp[i]);
        }
        res
    }
    pub fn bin_dec(bin: &Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        let mut pow: u32 = 0;
        let mut idx: usize = bin.len();

        while idx > 0 {
            idx -= 1;
            res += (bin[idx] * i32::pow(2, pow));
            pow += 1;
        }

        res
    }
}
