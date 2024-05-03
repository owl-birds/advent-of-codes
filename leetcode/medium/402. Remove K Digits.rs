use std::collections::VecDeque;
impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        // let mut num_order: Vec<[i32; 2]> = vec![]; // [num, order]
        // let mut temp: Vec<i32> = vec![];
        let mut temp: VecDeque<i32> = VecDeque::new();
        let mut idx: i32 = 0; // order
        for c in num.chars() {
            let num: i32 = c.to_digit(10).unwrap() as i32;
            temp.push_back(num);
            // num_order.push([num, idx]);
            idx += 1;
        }
        // println!("{:?}\n{:?}", temp, num_order);
        // println!("{:?}", temp);
        let mut count: i32 = 0;
        let mut idx: usize = 0;
        let mut new_num: Vec<i32> = vec![];

        while temp.len() > 0 {
            // println!("{:?}\n{:?}\n-----", new_num,temp);
            if new_num.len() > 0 && count < k {
                let front: i32 = *temp.front().unwrap();
                while count < k && new_num.len() > 0 && new_num[new_num.len() - 1] > front {
                    new_num.pop();
                    count += 1;
                }
            }
            if temp.len() == 1 {
                let first: i32 = temp.pop_front().unwrap();
                if count < k {
                    break;
                }
                new_num.push(first);
                break;
            }
            let first: i32 = temp.pop_front().unwrap();
            let second: i32 = temp.pop_front().unwrap();

            // println!("{};{}",first, second);
            if first > second && count < k {
                count += 1;
                temp.push_front(second);
                continue;
            }
            new_num.push(first);
            temp.push_front(second);
        }

        // println!("{:?}", new_num);
        // println!("{}:{}", count,k);
        while count < k - 1 {
            new_num.pop();
            count += 1;
        }
        // println!("{}:{}", count,k);

        let mut ans: String = String::new();
        let mut is_leading: bool = true;
        for i in 0..new_num.len() {
            if is_leading && new_num[i] == 0 {
                continue;
            }
            is_leading = false;
            ans.push(char::from_digit(new_num[i] as u32, 10).unwrap());
        }

        if new_num.len() == 0 || is_leading {
            return "0".to_string();
        }
        ans
    }
}
