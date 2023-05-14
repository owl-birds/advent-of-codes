use std::collections::HashSet;
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut collections_set: HashSet<String> = HashSet::new();
        Self::backtrack(&mut result, &mut Vec::<i32>::new(), &mut Vec::<usize>::new(), &nums, &mut collections_set);

        // // TEST
        // println!("{}", Self::is_same_arr(&vec![1,2,3,4,5], &vec![1,2,3,4]));
        // println!("{}", Self::is_same_arr(&vec![1,2,3,4,5], &vec![1,2,3,4,5]));
        // println!("{}", Self::is_same_arr(&vec![1,2,3,4,5], &vec![1,7,3,4,5]));
        // // TEST


        result
    }
    pub fn backtrack(result: &mut Vec<Vec<i32>>, temp_vec: &mut Vec<i32>, temp_idxs: &mut Vec<usize>, nums: &Vec<i32>, collections_set: &mut HashSet<String>) -> (){
        if temp_vec.len() == nums.len(){
            // check is exist
            //
            // let mut is_exist: bool = false;
            //
            let mut temp_str: String = String::from("");
            for i in 0..temp_vec.len(){
                temp_str.push_str(&format!("{}", temp_vec[i])[..]);
            }
            let mut is_exist: bool = collections_set.contains(&temp_str);

            // SLOW
            // for i in 0..result.len(){
            //     if Self::is_same_arr(&result[i], &temp_vec){is_exist = true; break;}
            // }
            if !is_exist{
                // println!("{}", temp_str);
                collections_set.insert(temp_str);
                result.push(temp_vec.to_vec());
            }
            return
        }
        for i in 0..nums.len(){
            if temp_idxs.contains(&i){continue}
            temp_vec.push(nums[i]);
            temp_idxs.push(i);
            Self::backtrack(result, temp_vec, temp_idxs, nums, collections_set);
            temp_vec.pop();
            temp_idxs.pop();
        }
    }
    pub fn is_same_arr(first_arr: &Vec<i32>, second_arr: &Vec<i32>) -> bool{
        if first_arr.len() != second_arr.len(){return false}
        
        for i in 0..first_arr.len(){
            if first_arr[i] != second_arr[i]{return false}
        }

        true
    }
}