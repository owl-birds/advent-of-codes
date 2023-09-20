impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        // [1, 1, 4, 2, 3]
        // arr_sum = 11
        // x = 5

        // -> we need to find the subarr with the sum of (arr_sum - x)
        // arr_sum - x = 11 - 5 = 6
        // [1, 1, 4, 2, 3] ---> 11 - 5
        // [x, x, 4, 2, x] ---> the sum of the subarr is 6, and its the min length
        // sliding window

        let mut sum: i32 = 0;
        let mut prefix_sum: Vec<i32> = vec![];
        for num in &nums {
            sum += *num;
            prefix_sum.push(sum);
        }
        if sum == x {
            return nums.len() as i32;
        }
        let subarr_sum: i32 = sum - x;
        let mut subarr_length: i32 = -1;
        // println!("nums:{:?}\nsum:{}\narr_sum-x or (subarr_sum):{}", nums, sum, subarr_sum);

        // // PREFIX SUM ::: TLE
        // let mut right: usize = prefix_sum.len();
        // // println!("prefix_sum:{:?}", prefix_sum);
        // while right > 0 {
        //     right -= 1;
        //     if prefix_sum[right] < subarr_sum {break}
        //     let mut left: usize = 0;
        //     if prefix_sum[right] == subarr_sum {
        //         subarr_length = if subarr_length > (right + 1) as i32 {subarr_length} else {(right + 1) as i32};
        //     }
        //     while left < right {
        //         let curr_sum: i32 = prefix_sum[right] - prefix_sum[left];
        //         if curr_sum < subarr_sum {break}
        //         if (curr_sum == subarr_sum) {
        //             subarr_length = if subarr_length > (right - left) as i32 {subarr_length} else {(right - left) as i32};
        //         }
        //         left += 1;
        //     }
        // }
        // if subarr_length == - 1 {
        //     return -1;
        // }
        // nums.len() as i32 - subarr_length
        // // PREFIX SUM

        // why dont we combine prefix sum and sliding window, nah it take more memory

        // SLIDING WINDOW
        let mut left: usize = 0;
        let mut right: usize = 0;
        let mut temp_sum: i32 = 0;
        // let mut temp_sum: i32 = 0;
        // sliding window

        // exs
        // [1,1,4,2,3]
        // subarr_sum that we need to find is 6 (11 - 5) : nums_sum - x : 11 - 5
        // temp_sum = 0
        // subarr_length = -1;
        // right = 0;
        // left = 0;

        // loop
        // right ++
        // temp_sum += 1
        // temp_sum += 1
        // temp_sum += 4
        // curr temp_sum = 6; equal to subarr_sum

        // if equal
        // compare the length of the current subarr_sum to the prev value in subarr_length
        // we need to find the longest
        // subarr_lenght = 3

        // loop
        // right += 1
        // temp_sum += 2
        // curr temp_sum = 8; bigger then subarr_sum
        // we enter the inner loop
        // inner loop
        // left += 1;
        // temp_sum -= nums[left]; nums[0] -> 7
        // temp_sum -= nums[left]; nums[1] -> 6
        // temp_sum -= nums[left]; nums[2] -> 2
        // exist; curr subarr_sum is less then subarr_sum

        // continue till right == nums.len();
        // .
        // .
        // .

        // exs

        while right < nums.len() {
            temp_sum += nums[right];

            // if we find sum of the current subarr bigger then subarr_sum_leftover
            while left < nums.len() && temp_sum > subarr_sum {
                temp_sum -= nums[left];
                left += 1;
            }

            // if the sum of the current subarr is equal to subarr_sum_leftover
            // then we compare the curr subarr length to the prev(if exist)

            // we need to find the subarr, that equal to the left over sum (nums_sum - x)
            // then we need to find the longest
            if temp_sum == subarr_sum {
                // println!("r:{}->l:{}", right, left);
                subarr_length = if subarr_length > (right - left + 1) as i32 {
                    subarr_length
                } else {
                    (right - left + 1) as i32
                };
            }

            right += 1;
        }
        if subarr_length == -1 {
            return -1;
        }

        nums.len() as i32 - subarr_length
        // SLIDING WINDOW
    }
}
