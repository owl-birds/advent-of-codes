impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for i in 0..nums.len(){
            if nums[i] < 0{count += 1}
            if nums[i] == 0{return 0}
        }
        if count % 2 != 0{
            return -1
        }
        1        
    }
}