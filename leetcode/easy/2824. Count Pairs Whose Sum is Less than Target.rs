impl Solution {
    pub fn count_pairs(nums: Vec<i32>, target: i32) -> i32 {
        let mut count: i32 = 0;
        let mut temp: Vec<i32> = nums.to_vec();
        temp.sort_by(|a, b| a.cmp(&b));
        // println!("{:?}", temp);
        let mut i: usize = temp.len();
        while i > 0 {
            i -= 1;
            if i == 0 {
                break;
            }
            let mut j: usize = i;
            while j > 0 {
                j -= 1;
                if temp[i] + temp[j] < target {
                    count += (j + 1) as i32;
                    break;
                }
            }
            // println!("{}-{}", i,j);
        }
        // for i in 0..nums.len() {
        //     for j in i+1..nums.len() {
        //         if nums[i] + nums[j] < target {count += 1;}
        //     }
        // }

        count
    }
}
