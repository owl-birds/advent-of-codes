use std::collections::HashSet;
use std::iter::FromIterator;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        let mut smaller: HashSet<i32> = if nums1.len() >= nums2.len() {
            Self::new_i32_hashset(&nums2)
        } else {
            Self::new_i32_hashset(&nums1)
        };
        let mut bigger: HashSet<i32> = if nums1.len() > nums2.len() {
            Self::new_i32_hashset(&nums1)
        } else {
            Self::new_i32_hashset(&nums2)
        };
        // println!("{:?}\n{:?}", smaller, bigger);

        for num in smaller.iter() {
            if bigger.contains(&num) {
                result.push(*num)
            }
        }

        result
    }
    pub fn new_i32_hashset(vec: &[i32]) -> HashSet<i32> {
        HashSet::from_iter(vec.iter().cloned())
    }
}
