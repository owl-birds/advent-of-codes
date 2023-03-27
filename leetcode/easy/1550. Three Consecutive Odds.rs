impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        if (arr.len() < 3){
            return false;
        }
        for i in 0..arr.len()-2{
            if (arr[i] % 2 == 0){ continue;}
            let mut count_odds: i32 = 1;
            for j in 1..3{ 
                if (arr[i+j] % 2 == 0) {break;}
                else {count_odds += 1}
            }
            if (count_odds == 3){
                return true;
            }
        }
        return false;              
    }
}