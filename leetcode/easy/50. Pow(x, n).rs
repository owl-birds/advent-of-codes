// TLE
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut result: f64 = 1.0;
        for _i in 0..n.abs(){
            result *= x;
        }
        if n < 0{
            return 1.0/result;
        }
        return result;
    }
}