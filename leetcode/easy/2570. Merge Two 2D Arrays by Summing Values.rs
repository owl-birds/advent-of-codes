use std::collections::HashMap;
impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // let mut id_sum: HashMap<i32, i32> = HashMap::new();
        let mut merged: Vec<Vec<i32>> = vec![];

        // two pointer
        let mut p1: usize = 0;
        let mut p2: usize = 0;
        // let mut is_one: bool = if nums1[0][0] > nums2[0][0] {false} else {true};
        while p1 < nums1.len() && p2 < nums2.len() {
            if nums1[p1][0] < nums2[p2][0] {
                merged.push(vec![nums1[p1][0], nums1[p1][1]]);
                p1 += 1;
            } else if nums1[p1][0] > nums2[p2][0] {
                merged.push(vec![nums2[p2][0], nums2[p2][1]]);
                p2 += 1;
            } else {
                // merged
                merged.push(vec![nums1[p1][0], nums1[p1][1] + nums2[p2][1]]);
                p1 += 1;
                p2 += 1;
            }
        }
        while p1 < nums1.len() {
            merged.push(vec![nums1[p1][0], nums1[p1][1]]);
            p1 += 1;
        }
        while p2 < nums2.len() {
            merged.push(vec![nums2[p2][0], nums2[p2][1]]);
            p2 += 1;
        }
        // println!("p1:{}\np2:{}",p1,p2);

        // // using sorting
        // for i in 0..nums1.len() {
        //     if let Some(sum) = id_sum.get_mut(&nums1[i][0]) {
        //         *sum += nums1[i][1];
        //     } else {
        //         id_sum.insert(nums1[i][0], nums1[i][1]);
        //     }
        // }
        // for i in 0..nums2.len() {
        //     if let Some(sum) = id_sum.get_mut(&nums2[i][0]) {
        //         *sum += nums2[i][1];
        //     } else {
        //         id_sum.insert(nums2[i][0], nums2[i][1]);
        //     }
        // }
        // for (id, sum) in id_sum.iter() {
        //     merged.push(vec![*id, *sum]);
        // }
        // merged.sort_by(|a, b| a[0].cmp(&b[0]));
        // // test
        // println!("{:?}", id_sum);
        // // using sorting

        merged
    }
}
