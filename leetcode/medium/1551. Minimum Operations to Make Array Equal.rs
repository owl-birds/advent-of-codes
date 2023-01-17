impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        let mut count_operation: i32 = 0;
        if n == 1 {
            return count_operation;
        }
        let mid_num: i32 = (1 + ((n-1)*2+1))/2;
        for i in 0..n/2 as i32{
            count_operation += (mid_num - (i*2+1));
        }
        return count_operation;
    }
}