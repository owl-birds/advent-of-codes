impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candy: i32 = *candies.iter().max().unwrap();
        // creating a specific size vector
        let mut result: Vec<bool> = vec![false; candies.len()];
        for i in 0..candies.len(){
            let added: i32 = candies[i] + extra_candies;
            if added >= max_candy{
                result[i] = true;
            }
        }
        result
    }
}