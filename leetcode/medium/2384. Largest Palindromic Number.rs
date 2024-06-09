use std::collections::HashMap;
// use std::collections::VecDeque;
impl Solution {
    pub fn largest_palindromic(num: String) -> String {
        let mut num_freq: HashMap<i32, i32> = HashMap::new();
        let mut temp: Vec<i32> = vec![];
        let mut res: String = String::new();

        for c in num.chars() {
            temp.push(c.to_digit(10).unwrap() as i32);
        }
        for i in 0..temp.len() {
            if let Some(freq) = num_freq.get_mut(&temp[i]) {
                *freq += 1;
            } else {
                num_freq.insert(temp[i], 1);
            }
        }
        let mut is_middle: bool = false;
        let mut middle: i32 = temp[0];
        let mut num_included: Vec<i32> = vec![];

        for (n, f) in num_freq.iter() {
            if f % 2 != 0 {
                if !is_middle || *n > middle {
                    middle = *n;
                }
                is_middle = true;
            }
            if *f >= 2 {
                num_included.push(*n);
            }
        }
        num_included.sort_by(|a, b| b.cmp(&a));
        if num_included.len() == 0 && is_middle || num_included[0] == 0 && is_middle {
            res.push(char::from_digit(middle as u32, 10).unwrap());
        } else if num_included.len() == 1 && num_included[0] == 0 {
            res.push('0');
        } else {
            for i in 0..num_included.len() {
                let len = num_freq.get(&num_included[i]).unwrap();
                for j in 0..len / 2 {
                    res.push(char::from_digit(num_included[i] as u32, 10).unwrap());
                }
            }
            if is_middle {
                res.push(char::from_digit(middle as u32, 10).unwrap());
            }
            let mut i: usize = num_included.len();
            while i > 0 {
                i -= 1;
                let len = num_freq.get(&num_included[i]).unwrap();
                for j in 0..len / 2 {
                    res.push(char::from_digit(num_included[i] as u32, 10).unwrap());
                }
            }
        }

        // println!("{:?}\n{:?}\n{:?}-{}-{}", temp, num_freq, num_included, middle, is_middle);
        res
    }
}
