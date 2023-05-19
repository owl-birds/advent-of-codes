// TLE 239 / 312
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut two_sum_map: HashMap<i32, Vec<HashSet<usize>>> = HashMap::new();
        let mut temp_nums: Vec<i32> = nums.clone(); temp_nums.sort();
        let mut i: usize = 0;
        while i < temp_nums.len(){
            let mut j: usize = i + 1;
            while j < temp_nums.len(){
                let temp_set: HashSet<usize> = HashSet::from([i,j]);
                let two_sum: i32 = temp_nums[i] + temp_nums[j];
                if !two_sum_map.contains_key(&two_sum){
                    two_sum_map.insert(two_sum, vec![temp_set]);
                    j += 1;
                    continue;
                }
                if let Some(map) = two_sum_map.get_mut(&two_sum){
                    map.push(temp_set);
                }
                j += 1;
            }
            i += 1;
        }
        // println!("{:?} length:{}", two_sum_map, two_sum_map.len());

        let mut result: Vec<Vec<i32>> = vec![];
        // let mut result_set: HashSet<String> = HashSet::new(); 
        let mut i: usize = 0;
        while i < temp_nums.len(){
            let zero_num: i32 = temp_nums[i] * -1;
            match two_sum_map.get(&zero_num){
                None => println!("two sum not found"),
                Some(sets) => {
                    // println!("{:?}", sets);
                    for j in 0..sets.len(){
                        if sets[j].contains(&i){break}
                        // println!("{:?} {}", sets[j], i);
                        let mut temp_vec: Vec<i32> = vec![];
                        for idx in sets[j].iter(){
                            temp_vec.push(temp_nums[*idx]);
                        }
                        temp_vec.push(temp_nums[i]);
                        // checking if the num combinations is already exist or not
                        let mut is_exist: bool = false;
                        for k in 0..result.len(){
                            let result_k_set: HashSet<i32> = HashSet::from([result[k][0], result[k][1], result[k][2]]);
                            let temp_vec_set: HashSet<i32> = HashSet::from([temp_vec[0], temp_vec[1], temp_vec[2]]);
                            if result_k_set.len() != temp_vec_set.len() {continue;}
                            // println!("{:?} {:?}", result_k_set, temp_vec_set);
                            let mut count: u32 = 0;
                            for num in temp_vec_set.iter(){
                                if result_k_set.contains(num){count += 1;}
                            }
                            if count as usize == result_k_set.len(){ is_exist = true; break;}
                        }
                        if !is_exist {result.push(temp_vec);}
                    }
                }
            }
            // if two_sum_map.contains_key(&zero_num){
            //     // println!("{:?}", two_sum_map.get(&zero_num));
            //     match two_sum_map.get(&zero_num){
            //         None => println!("NONE, not FOUND"), // here if not found nothing 
            //         Some(sets) =>{
            //             println!("{:?}", sets);
            //         }
            //     }
            // }
            i += 1;
        }

        result
    }
}