impl Solution {
    // Recursive way
    pub fn add_digits(num: i32) -> i32 {
        // iterative way
        return Self::add_digits_2(num); 
        // Recursive way
        if num <= 9{return num;}
        let mut temp_num = num;
        let mut result = 0;
        while temp_num > 0{
            result += (temp_num%10);
            temp_num /= 10;
        }
        Self::add_digits(result)
    }
    // iterative way
    pub fn add_digits_2(num: i32)->i32{
        let mut is_one_digit = num < 10;
        if is_one_digit {return num;}
        let mut temp_num = num;
        let mut result = 0;
        while !is_one_digit{
            while temp_num > 0{
                result += (temp_num%10);    
                temp_num /= 10;
            }

            if result < 10{
                is_one_digit = true;
                continue;
            }
            temp_num = result;
            result = 0;
        }
        result
    }
}