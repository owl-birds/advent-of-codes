impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }
        let mut score: i32 = 0;
        let mut temp: Vec<i32> = tokens.to_vec();
        temp.sort_by(|a, b| a.cmp(&b));
        if power < temp[0] {
            return 0;
        }
        let mut left: usize = 0; // we take score from left
        let mut right: usize = temp.len() - 1; // we gain token from right
        let mut curr_pow: i32 = power;
        // println!("{:?}", temp);
        while left < temp.len() && right >= 0 {
            // println!("{:?}-{}-{}\nleft:{},right:{}\n{}", temp, score,curr_pow,left,right,temp[left]);
            if curr_pow >= temp[left] {
                curr_pow -= temp[left];
                left += 1;
                score += 1;
                continue;
            }
            if score <= 0 {
                break;
            }
            if curr_pow + temp[right] < temp[left] {
                break;
            }
            if left == right {
                break;
            }
            curr_pow += temp[right];
            right -= 1;
            score -= 1;
        }
        // println!("{}", curr_pow);
        score
    }
}
