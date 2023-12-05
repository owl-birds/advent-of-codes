impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut count_move: i32 = 0;
        let mut curr_pos: Vec<i32> = points[0].to_vec();
        for i in 1..points.len() {
            // println!("{:?}", curr_pos);
            let x_abs_diff: i32 = i32::abs(curr_pos[0] - points[i][0]);
            let y_abs_diff: i32 = i32::abs(curr_pos[1] - points[i][1]);
            count_move += (if x_abs_diff > y_abs_diff {
                x_abs_diff
            } else {
                y_abs_diff
            });
            curr_pos = points[i].to_vec();
            // println!("y:{}\nx:{}", y_abs_diff, x_abs_diff);
        }

        count_move
    }
}
