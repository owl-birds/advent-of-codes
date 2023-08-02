impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let mut left: i32 = 1;
        let mut right: i32 = 10_000_000;
        // Tests are generated such that the answer will not exceed 10^7 and hour will have at most two digits after the decimal point.

        while left < right {
            let mid: i32 = (left + right) / 2;
            let time: f64 = Self::total_time(mid, &dist);
            if time > hour {
                left = mid + 1;
                continue;
            }
            right = mid;
        }
        if Self::total_time(left, &dist) > hour {
            return -1;
        }
        left
    }
    pub fn total_time(speed: i32, dist: &Vec<i32>) -> f64 {
        let mut total: f64 = 0.0;
        for i in 0..dist.len() {
            let mut time: f64 = dist[i] as f64 / speed as f64;
            total += time;
            if dist[i] % speed != 0 && i != (dist.len() - 1) {
                total += (time.ceil() - time);
            }
        }
        total
    }
    pub fn max_speed(dist: &Vec<i32>, hour: f64) -> f64 {
        let mut total_dist: f64 = 0.0;
        for i in 0..dist.len() {
            total_dist += dist[i] as f64;
        }
        // println!("dist:{},hour:{}", total_dist, hour);
        // if total_dist % hour != 0.0 {
        //     return -1.0;
        // }
        total_dist / hour
    }
}
// println!("10000000 km/h time:{}", Self::total_time(10000000, &dist));
// println!("199000000 km/h time:{}", Self::total_time(199000000, &dist));
// println!("100000000 km/h time:{}", Self::total_time(100000000, &dist).floor());
// println!("100000000 km/h time:{}", Self::total_time(100000000, &dist));
// println!("10000000 km/h time:{}", Self::total_time(10000000, &dist));
// println!("1000 km/h time:{}", Self::total_time(1000, &dist));
// println!("10 km/h time:{}", Self::total_time(10, &dist));
// println!("5 km/h time:{}", Self::total_time(5, &dist));
// println!("4 km/h time:{}", Self::total_time(4, &dist));
// println!("3 km/h time:{}", Self::total_time(3, &dist));
// println!("2 km/h time:{}", Self::total_time(2, &dist));
// println!("1 km/h time:{}", Self::total_time(1, &dist));
// println!("max_speed:{}", Self::max_speed(&dist, hour));
