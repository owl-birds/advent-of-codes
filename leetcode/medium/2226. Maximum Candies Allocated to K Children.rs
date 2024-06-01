impl Solution {
    pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
        let mut temp: Vec<i32> = candies.to_vec();
        let mut res: i32 = 0;
        temp.sort_by(|a, b| a.cmp(&b));
        let mut left = 1;
        let mut right = temp[temp.len() - 1];
        // println!("{:?}", temp);

        while left <= right {
            let mid = (left + right) / 2;
            // println!("{}--- {}-{}", mid, left, right);
            if Self::can_allocate(&temp, mid, k) {
                left = mid + 1;
                res = mid;
                continue;
            }
            right = mid - 1;
        }
        res
    }
    pub fn can_allocate(candies: &Vec<i32>, allocation: i32, k: i64) -> bool {
        // println!("-----");
        let mut count: i64 = 0;
        for i in 0..candies.len() {
            count += (candies[i] / allocation) as i64;
            // println!("{} ---- {}", count, allocation);
        }
        // println!("-----");
        count >= k
    }
}
