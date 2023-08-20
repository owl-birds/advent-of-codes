impl Solution {
    pub fn fill_cups(amount: Vec<i32>) -> i32 {
        let mut temp: Vec<i32> = amount.to_vec();
        let mut time_sec: i32 = 0;
        temp.sort_by(|a, b| b.cmp(a));

        // You always want to fill up the two types of water with the most unfilled cups.
        while temp[0] > 0 {
            // println!("{:?} ->{}",temp,time_sec);
            if temp[1] > 0 {
                temp[0] -= 1;
                temp[1] -= 1;
                time_sec += 1;
                temp.sort_by(|a, b| b.cmp(a)); // maybe there are some better alternative here
                continue;
            }
            temp[0] -= 1;
            time_sec += 1;
            temp.sort_by(|a, b| b.cmp(a)); // maybe there are some better alternative here
        }

        time_sec
    }
}
