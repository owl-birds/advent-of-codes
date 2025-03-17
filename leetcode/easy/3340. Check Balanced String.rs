impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut temp: Vec<char> = num.chars().collect();
        let mut even_sum: i32 = 0;
        let mut odd_sum: i32 = 0;
        for i in 0..temp.len() {
            if (i + 1) % 2 == 0 {
                even_sum += char::to_digit(temp[i], 10).unwrap() as i32;
            } else {
                odd_sum += char::to_digit(temp[i], 10).unwrap() as i32;
            }
        }

        // println!("{:?}\neven: {}\nodd: {}\n{}", temp, even_sum, odd_sum, '2' as i32 + '3' as i32);

        even_sum == odd_sum
    }
}
