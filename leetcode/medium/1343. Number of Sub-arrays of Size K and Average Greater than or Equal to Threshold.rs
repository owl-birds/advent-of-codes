impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>, k: i32, threshold: i32) -> i32 {
        let mut count: i32 = 0;

        let mut window_sum: i32 = 0;
        let mut idx: usize = 0;

        while idx < k as usize {
            window_sum += arr[idx];
            idx += 1;
        }

        if window_sum / k >= threshold {
            count += 1
        }

        while idx < arr.len() {
            window_sum += arr[idx];
            window_sum -= arr[idx - k as usize];
            if window_sum / k >= threshold {
                count += 1
            }
            idx += 1;
        }

        count
    }
}
