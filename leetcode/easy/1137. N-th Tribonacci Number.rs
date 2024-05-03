impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n <= 2 {
            return 1;
        }
        Self::helper(0, 1, 1, 2, n)
    }
    pub fn helper(t0: i32, t1: i32, t2: i32, count: i32, n: i32) -> i32 {
        if count == n {
            return t2;
        }
        Self::helper(t1, t2, t0 + t1 + t2, count + 1, n)
    }
}
