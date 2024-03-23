use std::cmp::Ordering;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.len() <= 0 {
            return 0;
        }
        let mut count: i32 = 0;
        let mut temp: Vec<Vec<i32>> = vec![];
        for i in 0..points.len() {
            temp.push(points[i].to_vec());
        }
        // println!("{:?}", temp);
        temp.sort_by(|a, b| {
            if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        // println!("{:?}", temp);
        let mut curr_arrow: i32 = temp[0][1];
        for i in 1..temp.len() {
            if curr_arrow >= temp[i][0] && curr_arrow <= temp[i][1] {
                count += 1;
                continue;
            }
            curr_arrow = temp[i][1];
        }

        temp.len() as i32 - count
    }
}
