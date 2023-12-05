impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        if n == 1 {
            return 0;
        }
        (n/2) + Self::number_of_matches(if n % 2 == 0 {n/2} else {n/2 + 1})
    }
}