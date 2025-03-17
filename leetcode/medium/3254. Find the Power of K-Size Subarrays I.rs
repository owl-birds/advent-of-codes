impl Solution {
    pub fn results_array(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 1 {
            return nums.to_vec();
        }
        let mut res: Vec<i32> = vec![];

        let mut start: usize = 0;
        let mut end: usize = 0;
        let mut prev_num: i32 = nums[0];
        // let mut is_conseq: bool = true;
        let mut break_idx: i32 = -1; // remembering the first breaking index
        while start < nums.len() - (k - 1) as usize && end < nums.len() {
            if start == end {
                end += 1;
                continue;
            }

            // IMPORTANT : the idea is keeping track the breaking point of the consecutive nums
            // AND ONLY THE FIRST IDX
            if break_idx == -1 && nums[end] != (prev_num + 1) {
                break_idx = end as i32;
                // println!("#end:{}---start:{}---prev_num:{}", end, start, prev_num);
                // is_conseq = false;
            }
            prev_num = nums[end];
            // AT THE END
            if (end - start + 1) as i32 == k {
                // println!("1start:{}--end:{}---break_idx:{}---prev_num:{}", start, end, break_idx, prev_num);
                if break_idx == -1 {
                    res.push(nums[end]);
                } else {
                    res.push(-1);
                }
                start += 1;
                end = if break_idx != -1 {
                    break_idx as usize
                } else {
                    end
                };
                // IMPORTANT : the idea is keeping track the breaking point of the consecutive nums
                if end == start {
                    prev_num = nums[end]
                } else if break_idx == -1 {
                    prev_num = nums[end - 1]
                } else if break_idx as usize == start {
                    prev_num = nums[start];
                } else {
                    prev_num = nums[break_idx as usize - 1];
                }
                break_idx = -1;
                // println!("2start:{}--end:{}---break_idx:{}---prev_num:{}", start, end, break_idx, prev_num);
                // println!("----");
                // is_conseq = true;
                continue;
            }
            end += 1;
        }

        // for i in 0..nums.len() - (k-1) as usize {
        //     let mut temp: i32 = nums[i];
        //     let mut is_power: bool = true;
        //     for j in i..i+k as usize {
        //         if temp != nums[j] {
        //             is_power = false;
        //             break;
        //         }
        //         temp += 1;
        //     }
        //     if is_power {res.push(temp-1);}
        //     else {res.push(-1);}
        // }

        res
    }
}
