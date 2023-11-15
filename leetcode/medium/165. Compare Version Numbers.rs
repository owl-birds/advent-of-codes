impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        // let v1_c: Vec<char> = version1.chars().collect();
        // let v2_c: Vec<char> = version2.chars().collect();

        let mut v1: Vec<Vec<char>> = vec![];
        let mut v2: Vec<Vec<char>> = vec![];

        let mut temp: Vec<char> = vec![];

        for c in version1.chars() {
            if c == '.' {
                v1.push(temp.to_vec());
                temp = vec![];
                continue;
            }
            temp.push(c);
        }
        v1.push(temp.to_vec());
        temp = vec![];
        for c in version2.chars() {
            if c == '.' {
                v2.push(temp.to_vec());
                temp = vec![];
                continue;
            }
            temp.push(c);
        }
        v2.push(temp.to_vec());
        temp = vec![];

        // println!("v1:{:?}\nv2:{:?}", v1, v2);

        let mut v1_idx: usize = 0;
        let mut v2_idx: usize = 0;

        while v1_idx < v1.len() && v2_idx < v2.len() {
            let mut temp1: usize = 0;
            let mut temp2: usize = 0;
            // if v1[v1_idx][temp1].to_digit(10).unwrap() > v2[v2_idx][temp2].to_digit(10).unwrap() && v1_idx > 0 {
            //     return 1;
            // }
            // if v1[v1_idx][temp1].to_digit(10).unwrap() < v2[v2_idx][temp2].to_digit(10).unwrap() && v2_idx > 0 {
            //     return -1;
            // }
            // skipping leading zeros
            while temp1 < v1[v1_idx].len() && v1[v1_idx][temp1] == '0' {
                temp1 += 1;
            }
            while temp2 < v2[v2_idx].len() && v2[v2_idx][temp2] == '0' {
                temp2 += 1;
            }
            // println!("temp1_b:{}\ntemp2_b:{}", temp1, temp2);
            let mut multiply = (v1[v1_idx].len() - temp1 - 1) as u32;
            // println!("1_in:{}", multiply);
            let mut v1_num: i32 = 0;
            while temp1 < v1[v1_idx].len() {
                v1_num += i32::pow(10, multiply) * v1[v1_idx][temp1].to_digit(10).unwrap() as i32;
                multiply -= 1;
                temp1 += 1;
            }
            // println!("num1:{}", v1_num);
            let mut multiply = (v2[v2_idx].len() - temp2 - 1) as u32;
            // println!("2_in:{}", multiply);
            let mut v2_num: i32 = 0;
            while temp2 < v2[v2_idx].len() {
                v2_num += i32::pow(10, multiply) * v2[v2_idx][temp2].to_digit(10).unwrap() as i32;
                multiply -= 1;
                temp2 += 1;
            }
            // println!("num2:{}", v2_num);

            if v1_num > v2_num {
                return 1;
            }
            if v1_num < v2_num {
                return -1;
            }

            v1_idx += 1;
            v2_idx += 1;
        }
        // println!("v1:{}\nv2:{}", v1_idx, v2_idx);
        while v1_idx < v1.len() {
            let mut temp1: usize = 0;
            // skipping leading zeros
            while temp1 < v1[v1_idx].len() && v1[v1_idx][temp1] == '0' {
                temp1 += 1;
            }
            let mut multiply = (v1[v1_idx].len() - temp1 - 1) as u32;
            // println!("1_in:{}", multiply);
            let mut v1_num: i32 = 0;
            while temp1 < v1[v1_idx].len() {
                v1_num += i32::pow(10, multiply) * v1[v1_idx][temp1].to_digit(10).unwrap() as i32;
                multiply -= 1;
                temp1 += 1;
            }
            if v1_num != 0 {
                return 1;
            }
            v1_idx += 1;
        }
        while v2_idx < v2.len() {
            let mut temp2: usize = 0;
            // skipping leading zeros
            while temp2 < v2[v2_idx].len() && v2[v2_idx][temp2] == '0' {
                temp2 += 1;
            }
            let mut multiply = (v2[v2_idx].len() - temp2 - 1) as u32;
            // println!("2_in:{}", multiply);
            let mut v2_num: i32 = 0;
            while temp2 < v2[v2_idx].len() {
                v2_num += i32::pow(10, multiply) * v2[v2_idx][temp2].to_digit(10).unwrap() as i32;
                multiply -= 1;
                temp2 += 1;
            }
            if v2_num != 0 {
                return -1;
            }

            v2_idx += 1;
        }

        // println!("{}", '1'.to_digit(10).unwrap());
        // println!("{}", '2'.to_digit(10).unwrap());
        // println!("{}", '9'.to_digit(10).unwrap());

        0
    }
}
