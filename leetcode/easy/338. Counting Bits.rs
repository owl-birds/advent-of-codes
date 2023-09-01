impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for num in 0..n + 1 {
            let mut temp_num: i32 = num;
            let mut count: i32 = 0;
            while temp_num > 0 {
                if temp_num % 2 != 0 {
                    count += 1;
                }
                temp_num /= 2;
            }
            result.push(count);
        }

        result
    }
}
