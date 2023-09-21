impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() == 0 && nums2.len() == 0 {
            return 0.0;
        }
        if nums1.len() == 0 {
            if nums2.len() % 2 != 0 {
                return nums2[nums2.len() / 2] as f64;
            }
            return ((nums2[nums2.len() / 2] + nums2[nums2.len() / 2 - 1]) as f64) / 2.0;
        }
        if nums2.len() == 0 {
            if nums1.len() % 2 != 0 {
                return nums1[nums1.len() / 2] as f64;
            }
            return ((nums1[nums1.len() / 2] + nums1[nums1.len() / 2 - 1]) as f64) / 2.0;
        }
        let mut temp_arr: Vec<i32> = vec![];
        if nums1[nums1.len() - 1] <= nums2[0] {
            for num in nums1 {
                temp_arr.push(num);
            }
            for num in nums2 {
                temp_arr.push(num);
            }
            if temp_arr.len() % 2 != 0 {
                return temp_arr[temp_arr.len() / 2] as f64;
            }
            return ((temp_arr[temp_arr.len() / 2] + temp_arr[temp_arr.len() / 2 - 1]) as f64) / 2.0;
        }
        if nums1[0] >= nums2[nums2.len() - 1] {
            for num in nums2 {
                temp_arr.push(num);
            }
            for num in nums1 {
                temp_arr.push(num);
            }
            if temp_arr.len() % 2 != 0 {
                return temp_arr[temp_arr.len() / 2] as f64;
            }
            return ((temp_arr[temp_arr.len() / 2] + temp_arr[temp_arr.len() / 2 - 1]) as f64) / 2.0;
        }
        let mut idx_1: usize = 0;
        let mut idx_2: usize = 0;

        while idx_1 < nums1.len() && idx_2 < nums2.len() {
            if nums1[idx_1] < nums2[idx_2] {
                temp_arr.push(nums1[idx_1]);
                idx_1 += 1;
            } else if nums1[idx_1] > nums2[idx_2] {
                temp_arr.push(nums2[idx_2]);
                idx_2 += 1;
            } else {
                temp_arr.push(nums2[idx_2]);
                idx_2 += 1;
                temp_arr.push(nums1[idx_1]);
                idx_1 += 1;
            }
        }
        while idx_1 < nums1.len() {
            temp_arr.push(nums1[idx_1]);
            idx_1 += 1;
        }
        while idx_2 < nums2.len() {
            temp_arr.push(nums2[idx_2]);
            idx_2 += 1;
        }
        // println!("auxiliary arr: {:?}", temp_arr);

        if temp_arr.len() % 2 != 0 {
            return temp_arr[temp_arr.len() / 2] as f64;
        }
        ((temp_arr[temp_arr.len() / 2] + temp_arr[temp_arr.len() / 2 - 1]) as f64) / 2.0
    }
}
