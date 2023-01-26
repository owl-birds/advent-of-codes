impl Solution {
    pub fn common_factors(a: i32, b: i32) -> i32 {
        if a == 1 && b == 1{
            return 1;
        }
        let mut smaller_num: i32 = a;
        let mut count: i32 = 1;
        if a > b{
            smaller_num = b;
            if a % b == 0{
                count += 1;
            }
        }else{
            if b % a == 0{
                count += 1;
            }
        }

        for div in 2..smaller_num/2+1{
            if a % div == 0 && b % div == 0{
                count += 1;
            }
        }
        return count;
    }
}
