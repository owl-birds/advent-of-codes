impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        // if nums.len() == 1 {return 1}
        let mut longest: i32 = 0;

        let mut right: usize = 0;
        let mut left: usize = 0;

        while right < nums.len() {
            let mut temp: usize = left;

            while temp < right && (nums[right] & nums[temp]) == 0 {
                temp += 1;
            }
            if temp < right {
                // left = right; // wrong here waht about the number after temp_idx and before right_idx ?
                left = temp;
                // correcting the left side of the window
                while left < right && (nums[right] & nums[left]) != 0 {
                    left += 1;
                }
            }
            // println!("temp:{}--left:{}--right:{}--length:{}", temp, left, right, (right-left+1));

            longest = if ((right - left + 1) as i32) > longest {
                (right - left + 1) as i32
            } else {
                longest
            };
            right += 1;
        }

        // println!("{:?}=>start:{},end:{}=>is_nice:{}", nums, 1, 3, Self::is_nice(&nums, 1, 3));
        // println!("{:?}=>start:{},end:{}=>is_nice:{}", nums, 0, 2, Self::is_nice(&nums, 0, 2));

        longest
    }
    pub fn is_nice(nums: &Vec<i32>, start: usize, end: usize) -> bool {
        if nums.len() == 1 {
            return true;
        }
        for i in start..end + 1 {
            for j in i + 1..end + 1 {
                if nums[i] & nums[j] != 0 {
                    return false;
                }
            }
        }

        true
    }
}
