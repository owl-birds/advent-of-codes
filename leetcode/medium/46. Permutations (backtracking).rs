impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut temp_vec: Vec<i32> = vec![];
        // Self::backtrack(&mut result, &mut vec![1,2,3], nums);
        Self::backtrack(&mut result, &mut temp_vec, nums);

        result
    }
    pub fn backtrack(result: &mut Vec<Vec<i32>>, temp_vec: &mut Vec<i32>, nums: Vec<i32>) -> (){
        if temp_vec.len() == nums.len(){
            result.push(temp_vec.to_vec());
            return;
        }
        for i in 0..nums.len(){
            // let curr_num = nums[i];
            // if temp_vec.contains(&curr_num){continue}
            if temp_vec.contains(&nums[i]){continue} // will become problem cause we borrowed sonmething
            // temp_vec.push(curr_num);
            temp_vec.push(nums[i]);
            // recursive
            Self::backtrack(result, temp_vec, nums.to_vec()); // here
            // Self::backtrack(result, temp_vec, nums); // here borrowed
            temp_vec.remove(temp_vec.len()-1);
        }
        // // TEST
        // let mut new_vec: Vec<i32> = temp_vec.to_vec();
        // new_vec.push(99999);
        // temp_vec.push(3333);
        // result.push(new_vec);
        // result.push(temp_vec.to_vec());
        // // TEST
    }
    pub fn backtrack_2(result: &mut Vec<Vec<i32>>) -> (){
        
    }
}


// TY 
// https://leetcode.com/problems/permutations/solutions/3488456/understandable-backstraking-way-with-step-by-step/?page=2
// Start for BackStrak loop
// First For Loop(for [1,2,3])

// 1.[ 1 ] --> a) B( [ 1 ] ) -->( remove Last ) [ ]
// 2.[ 2 ] --> b) B( [ 2 ] ) -->( remove Last ) [ ]
// 3.[ 3 ] --> c) B( [ 3 ] ) -->( remove Last ) [ ]
// Second For Loop(for [1] , [2] , [3])

// a )[ 1 ] --> [ 1, 2 ] --> d ) B( [ 1, 2 ] ) -->( remove Last ) [ 1 ]
// a.1)[ 1 ] --> [ 1, 3 ] --> d.1) B( [ 1, 3 ] ) -->( remove Last ) [ 1 ]

// b )[ 2 ] --> [ 2, 1 ] --> e ) B( [ 2, 1 ] ) -->( remove Last ) [ 2 ]
// b.1)[ 2 ] --> [ 2, 3 ] --> e.1) B( [ 2, 3 ] ) -->( remove Last ) [ 2 ]

// c )[ 3 ] --> [ 3, 1 ] --> f ) B( [ 3, 1 ] ) -->( remove Last ) [ 3 ]
// c.1)[ 3 ] --> [ 3, 2 ] --> f.1) B( [ 3, 2 ] ) -->( remove Last ) [ 3 ]
// Third For Loop(for [1,2] , [1,3] , [2,1] , [2,3] [3,1], [3,2])

// d)[ 1, 2 ] --> [ 1, 2, 3 ] --> break;
// d.1)[ 1, 3 ] --> [ 1, 3, 2 ] --> break;

// e)[ 2, 1 ] --> [ 2, 1, 3 ] --> break;
// e.1)[ 2, 3 ] --> [ 2, 3, 1 ] --> break;

// f)[ 3, 1 ] --> [ 3, 1, 2 ] --> break;
// f.1)[ 3, 2 ] --> [ 3, 2, 1 ] --> break;