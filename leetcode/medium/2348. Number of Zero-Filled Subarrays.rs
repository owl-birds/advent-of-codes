impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        // Self::find_occurences(11)
        let mut occurences: i64 = 0;
        let mut temp_length: i64 = 0;
        for i in 0..nums.len()+1{
            if i == nums.len(){
                occurences += Self::find_occurences(temp_length);
                break;
            }
            if nums[i] == 0{
                temp_length += 1;
                continue;
            }
            occurences += Self::find_occurences(temp_length);
            temp_length = 0;
        }

        occurences
    }
    pub fn find_occurences(sub_arr_length: i64)->i64{
        if sub_arr_length == 0 {return 0;}
        return sub_arr_length * (1 + sub_arr_length) / 2;
        // length * (first_num + last_num) / 2
        // let mut sum: i64 = 0;
        // for num in 1..sub_arr_length+1{
        //     sum += num;
        // }
        // sum
    }
}
