use std::cmp::Ordering;
use std::collections::HashSet;
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut temp: Vec<Vec<i32>> = vec![];
        let mut widest: i32 = 0;
        let mut x_sets: HashSet<i32> = HashSet::new();

        for i in 0..points.len() {
            if x_sets.contains(&points[i][0]) {
                continue;
            }
            temp.push(points[i].to_vec());
            x_sets.insert(points[i][0]);
        }
        // println!("{:?}", temp);
        temp.sort_by(|a, b| {
            if a[0] != b[0] {
                return a[0].cmp(&b[0]);
            }
            a[1].cmp(&b[1])
        });
        // println!("{:?}", temp);

        for i in 1..temp.len() {
            let diff = temp[i][0] - temp[i - 1][0];
            if diff > widest {
                widest = diff;
            }
        }

        widest
    }
}
