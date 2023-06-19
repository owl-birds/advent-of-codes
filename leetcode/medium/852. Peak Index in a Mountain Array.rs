impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = arr.len() - 1;

        while left < right {
            let mid = (left + right) / 2;
            if arr[mid - 1] < arr[mid] && arr[mid] > arr[mid + 1] {
                return mid as i32;
            }
            if arr[mid - 1] > arr[mid] {
                right = mid;
                continue;
            }
            left = mid + 1;
        }

        left as i32
    }
}
