impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count: i32 = 0;

        // some prefix sum idea
        // sum(i,j)=sum(0,j)-sum(0,i), where sum(i,j) represents the sum of all the elements from index i to j-1. Can we use this property to optimize it.
        let mut prefix_sum: Vec<i32> = vec![];
        let mut temp_sum: i32 = 0;
        for i in 0..nums.len() {
            temp_sum += nums[i];
            prefix_sum.push(temp_sum);
        }
        // println!("{:?}", prefix_sum);
        let mut idx: usize = 0;

        while idx < prefix_sum.len() {
            if idx == 0 {
                if prefix_sum[idx] == k {
                    count += 1
                }
                idx += 1;
                continue;
            }
            if prefix_sum[idx] == k {
                count += 1
            }
            for i in 0..idx {
                if prefix_sum[idx] - prefix_sum[i] == k {
                    count += 1
                }
            }
            idx += 1;
        }
        count

        // // brute force : TLE
        // let mut starting_len: usize = 1;

        // while starting_len <= nums.len() {
        //     let mut idx: usize = 0;
        //     let mut sum: i32 = 0;
        //     while idx < starting_len {
        //         sum += nums[idx];
        //         idx += 1;
        //     }
        //     if sum == k {
        //         count += 1;
        //     }
        //     while idx < nums.len() {
        //         sum += nums[idx];
        //         sum -= nums[idx-starting_len];
        //         if sum == k {count += 1}
        //         idx += 1;
        //     }
        //     starting_len += 1;
        // }

        // count
        // // brute force
    }
}
