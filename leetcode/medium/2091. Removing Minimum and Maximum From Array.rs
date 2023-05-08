impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut max_idx: usize = 0;
        let mut min_idx: usize = 0;
        
        for i in 0..nums.len(){
            if nums[i] > nums[max_idx]{
                max_idx = i;
            }
            if nums[i] < nums[min_idx]{
                min_idx = i;
            }
        }
        if max_idx == min_idx {return 1}
        println!("min:{} max:{} length:{}", min_idx, max_idx, nums.len());


        let shorter = if max_idx > min_idx {min_idx} else {max_idx};
        let longer = if max_idx > min_idx {max_idx} else {min_idx};
        
        let inter_diff = (longer - shorter) as i32;
        let first_diff = (shorter + 1) as i32;
        let last_diff = (nums.len() - longer) as i32;

        
        println!("short:{} long:{} length:{} diff:{}", shorter, longer, nums.len(), inter_diff);

        let mut deletions_1: i32 = first_diff + if last_diff > inter_diff {inter_diff} else {last_diff};
        let mut deletions_2: i32 = last_diff + if first_diff > inter_diff {inter_diff} else {first_diff};

        println!("{} {}", deletions_1, deletions_2);

        if deletions_1 > deletions_2 {return deletions_2}
        deletions_1
    }
}