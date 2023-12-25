impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut targets: Vec<Vec<char>> = vec![];
        let mut min: i32 = i32::MAX;
        let mut s_len: usize = s.len();
        let mut s_chars: Vec<char> = s.chars().collect();

        // making the target alternating string
        let mut temp_c: char = '0';
        let mut is_alternating: bool = true;
        // let mut temp_vec: Vec<char> = vec![];
        let mut count: i32 = 0;
        for i in 0..s_len {
            if temp_c != s_chars[i] {
                count += 1
            }
            if is_alternating {
                // temp_vec.push(temp_c);
                temp_c = '1';
                is_alternating = !is_alternating;
                continue;
            }
            // temp_vec.push(temp_c);
            temp_c = '0';
            is_alternating = !is_alternating;
        }
        if count < min {
            min = count
        }
        // targets.push(temp_vec.to_vec());
        let mut temp_c: char = '1';
        let mut is_alternating: bool = true;
        // let mut temp_vec: Vec<char> = vec![];
        let mut count: i32 = 0;
        for i in 0..s_len {
            if temp_c != s_chars[i] {
                count += 1
            }
            if is_alternating {
                // temp_vec.push(temp_c);
                temp_c = '0';
                is_alternating = !is_alternating;
                continue;
            }
            // temp_vec.push(temp_c);
            temp_c = '1';
            is_alternating = !is_alternating;
        }
        if count < min {
            min = count
        }
        // targets.push(temp_vec.to_vec());

        // // CHECK
        // for i in 0..targets.len() {
        //     let mut count: i32 = 0;
        //     for j in 0..targets[i].len() {
        //         if targets[i][j] != s_chars[j] {count += 1}
        //     }
        //     if count < min {min = count}
        // }
        // // CHECK

        // TEST
        // println!("{:?}", targets);
        // TEST

        min
    }
}
