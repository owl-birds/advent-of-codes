impl Solution {
    pub fn good_days_to_rob_bank(security: Vec<i32>, time: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];
        if time == 0 {
            for day in 0..security.len() as i32 {
                result.push(day);
            }
            return result;
        }
        if security.len() as i32 - time < 0 {
            return result;
        }

        // 5,3,3,3,5,6,2
        // 0 1 2 3 0 0 1
        // 0 4 3 2 1 0 0

        let mut pre: Vec<i32> = vec![0; security.len()];
        for i in 1..pre.len() {
            if security[i] <= security[i - 1] {
                pre[i] = pre[i - 1] + 1; // if it is decreasing add one ?
            } else {
                pre[i] = 0; // otherwise reset to zero
            }
        }
        let mut post: Vec<i32> = vec![0; security.len()];
        let mut idx: usize = security.len() - 1;
        while idx >= 1 {
            idx -= 1;
            if security[idx] <= security[idx + 1] {
                post[idx] = post[idx + 1] + 1; // if it it is increasing add one
            } else {
                post[idx] = 0; // otherwise reset to zero
            }
        }

        println!("pre:{:?}", pre);
        println!("post:{:?}", post);

        for day in time as usize..security.len() - time as usize {
            if pre[day] >= time && post[day] >= time {
                result.push(day as i32)
            }
        }

        // // SLOW | TLE
        // for day in (time as usize)..security.len()-(time as usize) {
        //     if Self::is_good_day(&security, day, time) {
        //         result.push(day as i32);
        //     }
        // }
        // // SLOW | TLE

        result
    }

    // SLOW
    pub fn is_good_day(security: &Vec<i32>, day: usize, time: i32) -> bool {
        for i in day - (time as usize)..day {
            if security[i] < security[i + 1] {
                return false;
            }
        }
        for i in day..day + (time as usize) {
            if security[i] > security[i + 1] {
                return false;
            }
        }
        true
    }
}
