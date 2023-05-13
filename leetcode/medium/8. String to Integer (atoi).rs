use std::collections::HashSet;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let NUMS: HashSet<char> = HashSet::from(['1','2','3','4','5','6','7','8','9','0']);
        let ALPHABETS: HashSet<char> = HashSet::from(['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z']);
        let mut idx: usize = 0;
        let mut num_vec: Vec<char> = vec![];
        let s_vec: Vec<char> = s.chars().collect();

        while idx < s_vec.len(){
            if ALPHABETS.contains(&s_vec[idx]) || s_vec[idx] == '.'{
                return 0;
            }
            if NUMS.contains(&s_vec[idx]){
                break;
            }
            idx += 1;
        }
        if idx == s_vec.len(){return 0}
        let first_num: usize = idx;
        while idx < s_vec.len(){
            if !NUMS.contains(&s_vec[idx]){
                break;
            }
            num_vec.push(s_vec[idx]);
            idx += 1;
        }

        // cleaning starting zero numbers
        let mut idx: usize = 0;
        while idx < num_vec.len(){
            if idx+1 == num_vec.len(){break}
            if num_vec[idx] != '0'{break}
            num_vec.remove(idx);
            // idx += 1;
        }
        // println!("{}", idx);
        //

        // checking the pos/neg
        let mut is_negative: bool = if first_num == 0 {false} else{if s_vec[first_num-1] == '-'{true} else {false}};    
        let mut idx: i32 = first_num as i32;
        let mut w_count: i32 = 0;
        let mut minus_count: i32 = 0;
        let mut plus_count: i32 = 0;
        while idx >= 0{
            if idx == 0{
                if s_vec[idx as usize] == '+'{plus_count += 1}
                if s_vec[idx as usize] == '-'{minus_count += 1}
                if s_vec[idx as usize] == ' '{w_count += 1}
                break;
            }
            if s_vec[idx as usize] == '+'{plus_count += 1}
            if s_vec[idx as usize] == '-'{minus_count += 1}
            if s_vec[idx as usize] == ' '{w_count += 1}
            idx -= 1;
        }
        println!("the sign : {} {}\nw:{}\np:{}\nm:{}", idx, is_negative, w_count, plus_count, minus_count);
        // checking the signs\
        if (plus_count > 0 && minus_count > 0) || plus_count > 1 || minus_count > 1{return 0}
        if first_num > 0 && s_vec[first_num-1] == ' ' && (plus_count > 0 || minus_count > 0){return 0}

        
        // if the digits length bigger then 10
        if num_vec.len() > 10{
            if is_negative{
                return -2147483648;
            }else{
                return 2147483647;
            }
        }


        let mut num_str: String = String::from("");
        for i in 0..num_vec.len(){
            num_str.push(num_vec[i]);
        }
        if num_vec.len() < 10{
            let num_res: i32 = num_str.parse().unwrap();
            if is_negative{
                return -1 * num_res;
            }else{
                return num_res;
            }
        }

        // checking the i32 limit
        // [-2^31, 2^31 - 1]
        if num_vec.len() == 10{
            if is_negative{
                let lower_limit: Vec<char> = "2147483648".to_string().chars().collect();
                let mut count_same_digit: u32 = 0;
                for i in 0..num_vec.len(){
                    let limit_digit: i32 = lower_limit[i].to_string().parse().unwrap();
                    let num_digit: i32 = num_vec[i].to_string().parse().unwrap();
                    if num_digit == limit_digit{count_same_digit += 1}
                    if num_digit > limit_digit{
                        return -2147483648;
                    }
                    if i >= 0 && num_digit < limit_digit{break}
                }
                if count_same_digit == 10{return -2147483648}
            }else{
                let upper_limit: Vec<char> = "2147483647".to_string().chars().collect();
                for i in 0..num_vec.len(){
                    let limit_digit: i32 = upper_limit[i].to_string().parse().unwrap();
                    let num_digit: i32 = num_vec[i].to_string().parse().unwrap();
                    if num_digit > limit_digit{
                        return 2147483647;
                    }
                    if i >= 0 && num_digit < limit_digit{break}
                }
            }

        }
        
        // println!("{:?}\n{:?}", upper_limit, lower_limit);
        let num_res: i32 = num_str.parse().unwrap();
        if is_negative{
            return -1*num_res;
        }
        num_res

        
        

        // // TEST
        // let test_1: i32 = 2147483647;
        // let test_2: i32 = -2147483648;
        // let test_3: String = String::from("002313213");
        // let test_4: i32 = test_3.parse().unwrap();
        // let test_5: i64 = "00312765421732".to_string().parse().unwrap();
        // let test_6: i32 = "002147483647".to_string().parse().unwrap();
        // println!("num_vec {:?}", num_vec);
        // println!("num_str {}", num_str);
        // println!("is negative {}", is_negative);
        // println!("{}\n{}\n{}\n{}\n{}\n{}", test_1, test_2, test_3, test_4+9, test_5,test_6);
        // println!("{:?}", NUMS);
        // println!("{}\n{:?}", idx, s_vec);
        // // TEST
    }
}