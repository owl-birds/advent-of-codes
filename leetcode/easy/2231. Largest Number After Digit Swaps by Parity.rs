impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let mut nums: Vec<i32> = vec![];
        // [[num, idx], ...]
        let mut odd_nums: Vec<Vec<i32>> = vec![];
        let mut even_nums: Vec<Vec<i32>> = vec![];
        let mut num_length: i32 = 0;
        let mut temp_num: i32 = num;
        while temp_num > 0{
            num_length += 1;
            temp_num /= 10;
        }
        

        let mut result: i32 = 0;
        let mut temp_num: i32 = num;
        let mut temp_num_length: i32 = num_length;
        while temp_num > 0{
            let digit: i32 = temp_num%10;
            // nums.push(digit);
            if digit % 2 == 0{
                even_nums.push(vec![digit, temp_num_length-1]);
                nums.push(0);
            }
            else {
                odd_nums.push(vec![digit, temp_num_length-1]);
                nums.push(1);
            }
            temp_num_length -= 1;
            temp_num /= 10;
        } 
        nums.reverse();
        // sort even and odds
        // v.sort_by(|a, b| a.cmp(b));
        odd_nums.sort_by(|a, b| b[0].cmp(&a[0]));
        even_nums.sort_by(|a, b| b[0].cmp(&a[0]));
        println!("{:?}\nodd:{:?}\neven:{:?}", nums, odd_nums, even_nums);

        let mut multiplication_ten: i32 = i32::pow(10, (num_length-1) as u32);
        println!("{}", multiplication_ten);
        
        let mut odd_idx: usize = 0;
        let mut even_idx: usize = 0;

        for i in 0..nums.len(){
            if nums[i] == 1{
                result += (multiplication_ten * odd_nums[odd_idx][0]);
                odd_idx += 1;
            }else{
                result += (multiplication_ten * even_nums[even_idx][0]);
                even_idx += 1;
            }
            multiplication_ten /= 10;
        }

        result
    }
}