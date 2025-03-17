use std::collections::HashSet;
impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![0, 0];
        let mut nums1_set: HashSet<i32> = HashSet::new();
        let mut nums2_set: HashSet<i32> = HashSet::new();
        // making sets
        for i in 0..nums1.len() {
            nums1_set.insert(nums1[i]);
        }
        for i in 0..nums2.len() {
            nums2_set.insert(nums2[i]);
        }
        // checking the nums
        for i in 0..nums1.len() {
            if nums2.contains(&nums1[i]) {
                result[0] += 1;
            }
        }
        for i in 0..nums2.len() {
            if nums1.contains(&nums2[i]) {
                result[1] += 1;
            }
        }

        result
    }
}
