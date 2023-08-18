// use std::collections::HashMap;
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        // let mut window: Vec<i32> = vec![];

        // let mut w_map: HashMap<i32, i32> = HashMap::new();

        // inspiration : https://algo.monster/problems/sliding_window_maximum
        let mut mon_deq: Vec<usize> = vec![]; // monotoic dequeue : it force the queue, in descending order
        for i in 0..nums.len() {
            while mon_deq.len() > 0 && nums[mon_deq[mon_deq.len() - 1]] < nums[i] {
                mon_deq.pop();
            }
            mon_deq.push(i);
            // println!("{:?}->{}", mon_deq, nums[mon_deq[0]]);

            if mon_deq[0] == (i - k as usize) {
                mon_deq.remove(0);
            }

            if i >= (k as usize - 1) {
                result.push(nums[mon_deq[0]]);
            }
        }
        result

        // // SLOW ONE, but ACCEPTED
        // let mut window_max: i32 = nums[0];
        // for i in 0..k as usize {
        //     window.push(nums[i]);
        //     // if let Some(freq) = w_map.get_mut(&nums[i]) {
        //     //     *freq += 1;
        //     // }else {
        //     //     w_map.insert(nums[i], 1);
        //     // }
        //     if nums[i] > window_max {
        //         window_max = nums[i];
        //     }
        // }
        // // println!("{:?}", w_map);

        // result.push(window_max);

        // for i in k as usize..nums.len() {
        //     window.remove(0);
        //     window.push(nums[i]);
        //     // if let Some(freq) = w_map.get_mut(&nums[i]) {
        //     //     *freq += 1;
        //     // }else {
        //     //     w_map.insert(nums[i], 1);
        //     // }
        //     // let mut is_zero: bool = false;
        //     // if let Some(freq) = w_map.get_mut(&nums[i-k as usize]) {
        //     //     *freq -= 1;
        //     //     if *freq == 0 {
        //     //         is_zero = true;
        //     //     }
        //     // }
        //     // if is_zero {
        //     //     w_map.remove(&nums[i-k as usize]);
        //     // }

        //     if window_max == nums[i-k as usize] {
        //         // window_max = nums[i-k as usize + 1];
        //         window_max = window[0];
        //         // for (num, freq) in w_map.iter() {
        //         //     if *num > window_max {
        //         //         window_max = *num;
        //         //     }
        //         // }
        //         for j in 1..window.len() {
        //             if window_max < window[j] {
        //                 window_max = window[j];
        //             }
        //         }
        //     }else if window_max < nums[i] {
        //         window_max = nums[i];
        //     }
        //     result.push(window_max);
        //     // result.push(*window.iter().max().unwrap());
        // }

        // result
    }
}
