impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let mut positive_nums: Vec<i32> = Vec::new();
        let mut negative_nums: Vec<i32> = Vec::new();

        for i in 0..nums.len(){
            if nums[i] > 0{
                positive_nums.push(nums[i]);
                continue;
            }
            negative_nums.push(nums[i]);
        }
        println!("+:{:?}\n-:{:?}", positive_nums, negative_nums);

        let mut pos_idx: usize = 0;
        let mut neg_idx: usize = 0;
        let mut is_pos: bool = true;
        for i in 0..nums.len(){
            if is_pos{
                result.push(positive_nums[pos_idx]);
                pos_idx += 1;
                is_pos = false;
                continue;
            }
            result.push(negative_nums[neg_idx]);
            is_pos = true;
            neg_idx += 1;
        }
        result
    }
}