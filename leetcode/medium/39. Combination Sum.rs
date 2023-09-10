use std::collections::HashMap;
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        // BRUTEFORCE
        Self::backtrack(&candidates, &mut vec![], &mut result, 0, target);

        // println!("{}", Self::is_contains_same_nums(&vec![3,2,2], &vec![2,3,2]));

        result
    }
    pub fn backtrack(
        candidates: &Vec<i32>,
        comb: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
        sum: i32,
        target: i32,
    ) {
        if sum > target {
            return;
        }
        if sum == target {
            for i in 0..result.len() {
                if Self::is_contains_same_nums(&result[i], comb) {
                    return;
                }
            }
            result.push(comb.to_vec());
            return;
        }
        for i in 0..candidates.len() {
            comb.push(candidates[i]);
            Self::backtrack(
                candidates,
                &mut comb.to_vec(),
                result,
                sum + candidates[i],
                target,
            );
            comb.pop();
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
