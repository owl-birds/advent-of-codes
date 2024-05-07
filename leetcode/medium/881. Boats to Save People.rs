impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut temp: Vec<i32> = people.to_vec();
        let mut min: i32 = 0;
        temp.sort_by(|a, b| a.cmp(&b));
        // println!("{:?}\n--------", temp);
        let mut left: usize = 0;
        let mut right: usize = temp.len() - 1;
        let mut is_left: bool = true;

        while left <= right {
            // println!("{} -- {}", left, right);
            let mut curr_boat: i32 = limit; // only two people at a time
            if right > 0 && curr_boat >= temp[right] {
                curr_boat -= temp[right];
                if right == 0 {
                    break;
                }
                right -= 1;
            }
            // println!("{} -- {}", left, right);
            if left < temp.len() && curr_boat >= temp[left] {
                curr_boat -= temp[left];
                left += 1;
            }
            // println!("{} -- {}", left, right);
            min += 1;
            // println!("{}", min);
            // println!("----");
        }

        min
    }
}
