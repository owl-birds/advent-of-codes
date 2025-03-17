impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        if arr.len() == 0 {
            return 0;
        }
        let mut count: i32 = 0;
        let mut prev_sum: Vec<i32> = vec![0; arr.len()];
        let mut odd_count: Vec<i32> = vec![0; arr.len()];
        let mut even_count: Vec<i32> = vec![0; arr.len()];
        let mut curr_odd: i32 = 0;
        let mut curr_even: i32 = 0;
        prev_sum[0] = arr[0];
        if arr[0] % 2 == 0 {
            even_count[0] += 1;
            curr_even += 1;
        } else {
            odd_count[0] += 1;
            count += 1;
            curr_odd += 1;
        }
        for i in 1..arr.len() {
            prev_sum[i] = prev_sum[i - 1] + arr[i];
            if prev_sum[i] % 2 == 0 {
                curr_even += 1;
                even_count[i] = curr_even;
                odd_count[i] = curr_odd;
            } else {
                curr_odd += 1;
                odd_count[i] = curr_odd;
                even_count[i] = curr_even;
            }
        }
        // println!("even: {:?}", even_count);
        // println!("odd: {:?}", odd_count);
        // println!("prev_sum: {:?}", prev_sum);

        for i in 1..prev_sum.len() {
            if prev_sum[i] % 2 == 0 {
                count = (count + odd_count[i - 1]) % (i32::pow(10, 9) + 7);
                continue;
            }
            count += 1;
            count = (count + even_count[i - 1]) % (i32::pow(10, 9) + 7);
        }

        count
    }
}
