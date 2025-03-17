impl Solution {
    pub fn is_good(nums: Vec<i32>) -> bool {
        let mut temp: Vec<i32> = nums.to_vec();
        temp.sort_by(|a, b| b.cmp(&a));
        if temp[temp.len() - 1] != 1 {
            return false;
        }
        if temp[0] != temp.len() as i32 - 1 || temp[1] != temp.len() as i32 - 1 {
            return false;
        }
        let mut curr: i32 = temp.len() as i32 - 1;
        for i in 2..temp.len() {
            curr -= 1;
            if temp[i] != curr {
                return false;
            }
        }

        // println!("{:?}", temp);
        true
    }
}
