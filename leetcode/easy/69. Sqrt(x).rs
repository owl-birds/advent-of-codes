impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 1 || x == 0{
            return x;
        }
        let mut start: i64 = 1;
        let mut end: i64 = (x/2+1) as i64;

        while start < end{
            let mid: i64 = start + (end-start)/2;
            if mid*mid > x as i64{
                end = mid;
                continue;
            }
            start = mid + 1;
        }

        return (start-1) as i32;
    }
}