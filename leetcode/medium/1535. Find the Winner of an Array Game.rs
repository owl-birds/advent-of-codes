use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        if k > arr.len() as i32 {
            let max_num = *arr.iter().max().unwrap();
            return max_num;
        }
        // let mut temp_arr: Vec<i32> = arr.to_vec();
        let mut deque_arr: VecDeque<i32> = VecDeque::new();
        for i in 0..arr.len() {
            deque_arr.push_back(arr[i]);
        }
        let mut num_win: HashMap<i32, i32> = HashMap::new();
        let mut max_win: i32 = 0;
        let mut num: i32 = i32::MAX;

        while max_win != k && arr.len() >= 2 {
            // let first_num = temp_arr.remove(0);
            let first_num = deque_arr.pop_front().unwrap();
            let second_num = deque_arr.pop_front().unwrap();

            // if temp_arr[0] > temp_arr[1] {
            // if first_num > temp_arr[0] {
            if first_num > second_num {
                if let Some(win) = num_win.get_mut(&first_num) {
                    if *win + 1 > max_win {
                        max_win = *win + 1;
                        num = first_num;
                    }
                    *win += 1;
                } else {
                    if 1 > max_win {
                        max_win = 1;
                        num = first_num;
                    }
                    num_win.insert(first_num, 1);
                }
                // let removed_num = temp_arr.remove(1);
                // temp_arr.push(temp_arr[0]);
                // temp_arr[0] = first_num;
                deque_arr.push_front(first_num);
                deque_arr.push_back(second_num);
            } else {
                // if let Some(win) = num_win.get_mut(&temp_arr[0]) {
                if let Some(win) = num_win.get_mut(&second_num) {
                    if *win + 1 > max_win {
                        max_win = *win + 1;
                        // num = temp_arr[0];
                        num = second_num;
                    }
                    *win += 1;
                } else {
                    if 1 > max_win {
                        max_win = 1;
                        num = second_num;
                    }
                    num_win.insert(second_num, 1);
                }
                // let removed_num = temp_arr.remove(0);
                // temp_arr.push(first_num);
                deque_arr.push_front(second_num);
                deque_arr.push_back(first_num);
            }
        }
        num
    }
}
