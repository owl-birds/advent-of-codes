// some enligtment shit
// Explanation of the following code snippet:

// if i > start and candidates[i]==candidates[i-1]:
//     continue

// 1.
// Why do we discard the elements (of equal values) when i > start and candidates[i]==candidates[i-1]?
// Reason:
// We do so to avoid duplicate combinations.
// When we've gone through the first element and the next elements are of the same value (when i > start and candidates[I]==candidates[i-1]), taking out (i-1)'th element and taking in i'th element will result in a duplicate combination. (example: candidates = [1,1,1,1]; here if our path is [1,1,1] and the next element is '1' as well, popping the last '1' out will give us path = [1,1], and then if we take the next '1' from the list, our path will become [1,1,1] again, which is a duplicate of the previous path. So we ignore it and continue our search for an element with a different value.

// 2.
// So why are we taking the 1st element even when it's equal to the previous one (why i > start and not i >= start)?
// Reason:
// We have to consider the 1st element (even if it's the same as before).
// If multiple elements of the same value exist in the candidates list (example: candidates = [10,1,2,7,6,1,5], target = 8; here we have two 1), we can pick up those elements with equal values in the same combination (the result of the example above is [ [1,1,6], [1,2,5], [1,7], [2,6] ] ). Hence, `i > start', we take the first (i'th) element (regardless of this being equal to the previous ones).

use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut temp_cands: Vec<i32> = candidates.to_vec();
        temp_cands.sort_by(|a, b| a.cmp(b));
        // let mut idx_set: HashSet<usize> = HashSet::new();
        // let mut idxs: Vec<usize> = vec![];

        // UNOPTIMISED SHIT :: need sorted nums/candidates --> NOW ITS OPTIMISED
        // TY : https://leetcode.com/problems/combination-sum-ii/solutions/16862/c-backtracking-solution-with-detailed-explanation/
        // println!("{:?}", temp_cands);
        Self::backtrack(
            &temp_cands,
            &mut vec![],
            &mut result, // , 0
            target,
            0, // , &mut idx_set, &mut idxs
        );
        result
    }
    pub fn backtrack(
        candidates: &Vec<i32>,
        comb: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>, // , sum: i32
        target: i32,
        curr_idx: usize, // , idx_set: &mut HashSet<usize>, idxs: &mut Vec<usize>
    ) {
        // println!("{:?}", comb);
        if 0 == target {
            // for i in 0..result.len() {
            //     if Self::is_contains_same_nums(&result[i], &comb) {return;}
            // }
            result.push(comb.to_vec());
            return;
        }
        for i in curr_idx..candidates.len() {
            // if idx_set.contains(&i) {
            //     continue;
            // }
            if i > 0 && candidates[i] == candidates[i - 1] && i > curr_idx {
                continue;
            } // check duplicate combination
            if candidates[i] > target {
                return;
            }
            comb.push(candidates[i]);
            // idx_set.insert(i);
            // idxs.push(i);
            Self::backtrack(
                candidates,
                comb,
                result, // , sum + candidates[i]
                target - candidates[i],
                i + 1, // , idx_set, idxs
            );
            comb.pop();
            // if let Some(poped_idx) = idxs.pop() {
            //     idx_set.remove(&poped_idx);
            // }
        }
    }
    pub fn is_contains_same_nums(nums1: &Vec<i32>, nums2: &Vec<i32>) -> bool {
        if nums1.len() != nums2.len() {
            return false;
        }
        let mut nums1_map: HashMap<i32, i32> = HashMap::new();
        let mut nums2_map: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums1.len() {
            // nums1
            if let Some(freq) = nums1_map.get_mut(&nums1[i]) {
                *freq += 1;
            } else {
                nums1_map.insert(nums1[i], 1);
            }
            // nums2
            if let Some(freq) = nums2_map.get_mut(&nums2[i]) {
                *freq += 1;
            } else {
                nums2_map.insert(nums2[i], 1);
            }
        }

        // println!("{:?}", nums1_map);
        // println!("{:?}", nums2_map);

        for (num1, freq1) in nums1_map.iter() {
            if let Some(freq2) = nums2_map.get(&num1) {
                if *freq1 == *freq2 {
                    continue;
                }
            }
            return false;
        }

        true
    }
}
