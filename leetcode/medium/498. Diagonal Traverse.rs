impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let R: i32 = mat.len() as i32;
        let C: i32 = mat[0].len() as i32;
        println!("col:{C} row:{R}");

        let mut curr_c: i32 = 0;
        let mut curr_r: i32 = 0;
        let mut position: Vec<i32> = vec![curr_c, curr_r];
        // let mut position: Vec<i32> = vec![2, 2];
        let mut is_up: bool = true;
        let mut result: Vec<i32> = vec![];

        while position[0] >= 0
            && position[1] >= 0
            && position[0] < C
            && position[1] < R
            && ((result.len() as i32) < (R * C))
        {
            // println!("starting:{:?},is_up:{}:num:{}", position, is_up, mat[position[1] as usize][position[0] as usize]);
            while (position[0] >= 0 && position[0] < C) && (position[1] >= 0 && position[1] < R) {
                // UP
                if is_up {
                    // println!("{:?}:num={}", position, mat[position[1] as usize][position[0] as usize]);
                    // println!("c:{},r:{}", position[0], position[1]);
                    result.push(mat[position[1] as usize][position[0] as usize]);
                    position[0] += 1;
                    position[1] -= 1;
                    continue;
                }
                // DOWN
                // println!("{:?}:num={}", position, mat[position[1] as usize][position[0] as usize]);
                // println!("c:{},r:{}", position[0], position[1]);
                result.push(mat[position[1] as usize][position[0] as usize]);
                position[0] -= 1;
                position[1] += 1;
            }
            is_up = !is_up;
            if is_up {
                // prev: DOWN
                // println!("prev-down=>end:{:?},is_up:{}", position, is_up);
                if position[0] == -1 && position[1] == (R - 1) {
                    position[0] += 1;
                } else if position[0] < C && position[1] == R {
                    position[0] += 2;
                    position[1] -= 1;
                } else if position[0] == -1 && position[1] < R {
                    position[0] += 1;
                }
                // TEST
                // println!("starting:{:?},is_up:{}:num:{}", position, is_up, mat[position[1] as usize][position[0] as usize]);
                // println!("prev-down=>new_starting:{:?},is_up:{}", position, is_up);
                // break;
                // TEST
                continue;
            }
            // prev: UP
            // println!("prev-up=>end:{:?},is_up:{}", position, is_up);
            if position[0] < C && position[1] == -1 {
                position[1] += 1;
            } else if position[0] == C && position[1] == -1 {
                position[0] -= 1;
                position[1] += 2;
            } else if position[0] == C && position[1] == 0 {
                position[1] += 2;
                position[0] -= 1;
            } else if position[0] == C && position[1] < R - 1 {
                position[0] -= 1;
                position[1] += 2;
            }
            // TEST
            // println!("starting:{:?},is_up:{}:num:{}", position, is_up, mat[position[1] as usize][position[0] as usize]);
            // println!("prev-up=>new_starting:{:?},is_up:{}", position, is_up);
            // break;
            // TEST
        }
        // println!("{:?}",result);
        result
    }
}
