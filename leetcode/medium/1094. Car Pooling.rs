use std::collections::HashMap;
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut temp_trips: Vec<Vec<i32>> = vec![];
        for i in 0..trips.len() {
            temp_trips.push(trips[i].to_vec());
        }
        temp_trips.sort_by(|a, b| a[1].cmp(&b[1]));
        // println!("{:?}", temp_trips);

        let mut dest_pass: HashMap<i32, i32> = HashMap::new();
        let mut curr_pass: i32 = 0;
        let mut curr_post: i32 = 0;
        let mut idx: usize = 0;
        // while curr_post < temp_trips[temp_trips.len()-1][2] {
        while idx < temp_trips.len() {
            // check if curr position is the destination of any passenger?
            if let Some(pass) = dest_pass.get_mut(&curr_post) {
                curr_pass -= *pass;
                *pass = 0;
                if *pass == 0 {
                    dest_pass.remove(&curr_post);
                }
            }
            //
            if temp_trips[idx][1] == curr_post {
                curr_pass += temp_trips[idx][0];
                if curr_pass > capacity {
                    return false;
                }
                if let Some(pass) = dest_pass.get_mut(&temp_trips[idx][2]) {
                    *pass += temp_trips[idx][0];
                } else {
                    dest_pass.insert(temp_trips[idx][2], temp_trips[idx][0]);
                }
                idx += 1;
            }
            if idx < temp_trips.len() && temp_trips[idx][1] == curr_post {
                continue;
            }
            curr_post += 1;
            // println!("{:?}, curr_pass:{}, curr_post:{}", dest_pass, curr_pass, curr_post);
        }

        true
    }
}
