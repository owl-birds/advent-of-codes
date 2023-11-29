impl Solution {
    pub fn max_coins(piles: Vec<i32>) -> i32 {
        // let mut alice: i32 = 0;
        let mut me: i32 = 0;
        let mut bob: i32 = 0;
        let mut temp: Vec<i32> = piles.to_vec();
        temp.sort_by(|a, b| b.cmp(&a));
        let mut n: i32 = temp.len() as i32;
        // println!("{:?}", temp);

        let mut b_i: usize = temp.len() - 0;
        let mut idx: usize = 1;
        while idx < b_i {
            me += temp[idx];
            idx += 2;
            b_i -= 1;
        }

        me
    }
}
