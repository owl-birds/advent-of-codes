// use std::collections::HashSet;
impl Solution {
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        // let mut num1_set: HashSet<i32> = HashSet::new();
        let mut result: i32 = -1;

        // for i in 0..nums1.len() {
        //     num1_set.insert(nums1[i]);
        // }
        // for i in 0..nums2.len() {
        //     if num1_set.contains(&nums2[i]) {
        //         result = nums2[i];
        //         break;
        //     }
        // }

        let mut i1: usize = 0;
        let mut i2: usize = 0;

        while i1 < nums1.len() && i2 < nums2.len() {
            if nums1[i1] == nums2[i2] {
                result = nums1[i1];
                break;
            }
            if nums1[i1] > nums2[i2] {
                i2 += 1;
                continue;
            }
            i1 += 1;
        }

        result
    }
}
