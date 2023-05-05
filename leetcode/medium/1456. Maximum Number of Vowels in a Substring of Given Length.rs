use std::collections::HashSet;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut count_vowels: i32 = 0;
        let mut max_vowels: i32 = 0;
        let vowels_set: HashSet<char> = HashSet::from(['a','e','i','o','u']);


        let mut s_vec: Vec<char> = vec![];
        for c in s.chars(){
            s_vec.push(c);
        }
        // println!("{:?}", s_vec);
        
        // optimised
        let mut idx: usize = 0;        
        while idx < k as usize{
            if vowels_set.contains(&s_vec[idx]){
                count_vowels += 1;
            }
            idx += 1;
        }
        max_vowels = count_vowels;
        // println!("{} {}", idx, count_vowels);

        while idx < s_vec.len(){
            // println!("{}", idx);
            if vowels_set.contains(&s_vec[idx]){
                count_vowels += 1;
            }
            if vowels_set.contains(&s_vec[idx-k as usize]){
                count_vowels -= 1;
            }
            if count_vowels > max_vowels{
                max_vowels = count_vowels;
            }
            idx += 1;
        }
        // println!("max : {}", max_vowels);

        // // TLE
        // let mut idx: usize = 0;        
        // while idx < s_vec.len() - k as usize + 1{
        //     //
        //     let mut count_vowels: i32 = 0;
        //     //
            
        //     // here can be optimised
        //     // like keeping track the count, track the vowels for the first k chars
        //     for temp_idx in idx..idx+k as usize{
        //         if vowels_set.contains(&s_vec[temp_idx]){
        //             count_vowels += 1;
        //         }
        //     }
        //     // println!("{}", count_vowels);
        //     if count_vowels > max_vowels{
        //         max_vowels = count_vowels;
        //     }
        //     idx += 1;
        // }


        max_vowels
    }
}