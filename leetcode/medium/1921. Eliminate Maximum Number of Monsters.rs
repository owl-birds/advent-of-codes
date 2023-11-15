use std::collections::HashSet;
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut removed: HashSet<usize> = HashSet::new();
        let mut dist_speed: Vec<[i32; 2]> = vec![];
        let mut idx_time: Vec<[i32; 2]> = vec![];

        for i in 0..dist.len() {
            let diff = dist[i] - speed[i];
            dist_speed.push([dist[i], speed[i]]);
            idx_time.push([
                i as i32,
                if dist[i] % speed[i] != 0 {
                    (dist[i] / speed[i]) + 1
                } else {
                    dist[i] / speed[i]
                },
            ]);
        }
        idx_time.sort_by(|a, b| a[1].cmp(&b[1]));
        // println!("dist_speed:{:?}\nidx_time:{:?}",dist_speed, idx_time);

        let mut count_eliminations: i32 = 1;
        let mut curr_time: i32 = 1;
        let mut idx: usize = 1;

        while idx < idx_time.len() {
            if idx_time[idx][1] <= curr_time {
                break;
            }
            count_eliminations += 1;
            curr_time += 1;
            idx += 1;
        }

        count_eliminations
    }
}
