// use std::collections::HashSet;
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        // let mut temp_nums: Vec<i32> = nums.to_vec();
        // temp_nums.sort();

        // SLOW AS HECK
        // Self::backtrack(&mut result, &mut Vec::<i32>::new(), &nums, 0);
        Self::backtrack_2(0, &mut result, &mut Vec::<i32>::new(), &nums);

        result
    }
    pub fn backtrack(result: &mut Vec<Vec<i32>>, temp_vec: &mut Vec<i32>, nums: &Vec<i32>, start: usize) -> (){
        // result.push(temp_vec.to_vec());
        let mut is_exist: bool = false;
        for i in 0..result.len(){
            if result[i].len() != temp_vec.len(){continue}
            // let mut temp_set = HashSet::new();
            // for j in 0..result[i].len(){
            //     temp_set.insert(result[i][j]);
            // }
            let mut count: u32 = 0;
            for j in 0..temp_vec.len(){
                // if temp_set.contains(&temp_vec[j]){count += 1}
                if result[i].contains(&temp_vec[j]){count += 1}
            }
            if count == temp_vec.len() as u32{is_exist=true; break;}
        }
        if !is_exist{
            result.push(temp_vec.to_vec());
        }
        // for i in 0..nums.len(){ // so many redudancy
        for i in start..nums.len(){
            if temp_vec.contains(&nums[i]){continue}
            temp_vec.push(nums[i]);
            // Self::backtrack(result, temp_vec, nums.to_vec(), start+1);
            Self::backtrack(result, temp_vec, nums, start+1);
            // temp_vec.remove(temp_vec.len()-1);
            temp_vec.pop();
        }
    }
    pub fn backtrack_2(i: usize, result: &mut Vec<Vec<i32>>, temp_vec: &mut Vec<i32>, nums: &Vec<i32>) {
        if i == nums.len() {
            // println!("{:?}", temp_vec);
            result.push(temp_vec.to_vec());
            return
        }
        // use it
        temp_vec.push(nums[i]);
        Self::backtrack_2(i+1, result, temp_vec, nums);
        temp_vec.pop();
        // skip it
        Self::backtrack_2(i+1, result, temp_vec, nums);
    }
}