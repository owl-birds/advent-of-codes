impl Solution {
    pub fn count_digits(num: i32) -> i32 {
        let mut temp_num: i32 = num;
        let mut count: i32 = 0;

        while temp_num > 0 {
            let digit = temp_num % 10;
            if num % digit == 0 {
                count += 1;
            }
            temp_num /= 10;
        }

        count
    }
}
