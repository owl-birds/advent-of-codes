use std::collections::HashMap;
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let sqrt_c: i64 = (c as f64).sqrt() as i64;
        // faster solution
        let mut start: i64 = 0;
        let mut end: i64 = sqrt_c;

        while start <= end{
            let sum_square: i64 = start*start + end*end;
            if sum_square > c as i64{
                end -= 1;
                continue;
            }
            if sum_square < c as i64{
                start += 1;
                continue;
            }
            return true;
        }
        return false;

        // slow solution
        let mut sqr_1: HashMap<i32, i32> = HashMap::new();
        for num_1 in 0..(sqrt_c + 1) as i32{
            sqr_1.insert(num_1*num_1, num_1);
        }
        for num_2 in 0..(sqrt_c + 1) as i32{
            let diff_sqr_2: i32 = c - (num_2*num_2);
            if sqr_1.contains_key(&diff_sqr_2){
                return true;
            }
        }
        return false;
    }
}
