impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let mut start: i64 = 1;
        let mut end: i64 = (*time.iter().max().unwrap()) as i64 * total_trips as i64;
        // println!("{}", Self::is_enough(&time, 11, total_trips));

        while start < end{
            let mid = (start+end)/2;
            if Self::is_enough(&time, mid, total_trips){
                end = mid;
                continue;
            }
            start = mid + 1;
        }

        return start;
    }
    pub fn is_enough(times: &Vec<i32>, time: i64, target_trips: i32)->bool{
        let mut count_trips: i64 = 0;
        for t in times{
            count_trips += time/(*t as i64);
        }
        count_trips >= (target_trips as i64)
    }
}
