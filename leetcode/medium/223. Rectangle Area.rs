use std::collections::HashSet;
impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let mut area: i32 = ((ax2 - ax1) * (ay2 - ay1)) + ((bx2 - bx1) * (by2 - by1));
        let mut xs: Vec<i32> = vec![ax1, ax2, bx1, bx2];
        let mut ys: Vec<i32> = vec![ay1, ay2, by1, by2];
        // println!("xs:{:?}\nys:{:?}", xs, ys);
        let mut int_xs: HashSet<i32> = HashSet::new();
        let mut int_ys: HashSet<i32> = HashSet::new();

        for i in 2..xs.len() {
            if xs[0] <= xs[i] && xs[1] >= xs[i] {
                int_xs.insert(xs[i]);
            }
        }
        for i in 0..2 {
            if xs[2] <= xs[i] && xs[3] >= xs[i] {
                int_xs.insert(xs[i]);
            }
        }
        for i in 2..ys.len() {
            if ys[0] <= ys[i] && ys[1] >= ys[i] {
                int_ys.insert(ys[i]);
            }
        }
        for i in 0..2 {
            if ys[2] <= ys[i] && ys[3] >= ys[i] {
                int_ys.insert(ys[i]);
            }
        }
        let mut int_length: i32 = 0;
        let mut int_width: i32 = 0;
        if int_xs.len() == 2 && int_ys.len() == 2 {
            let mut temp: Vec<i32> = vec![];
            for x in int_xs.iter() {
                temp.push(*x);
            }
            temp.sort_by(|a, b| a.cmp(&b));
            int_length = temp[1] - temp[0];
            let mut temp: Vec<i32> = vec![];
            for y in int_ys.iter() {
                temp.push(*y);
            }
            temp.sort_by(|a, b| a.cmp(&b));
            int_width = temp[1] - temp[0];
        }

        // println!("----------\nxs:{:?}\nys:{:?}", int_xs, int_ys);
        // println!("int_length:{} int_width:{}", int_length, int_width);
        area - (int_length * int_width)
    }
}
