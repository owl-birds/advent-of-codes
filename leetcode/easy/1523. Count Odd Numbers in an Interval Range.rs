impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        // what u re supposed to do
        if low % 2 == 0 || high % 2 == 0{
            return (high-low + 1)/2;
        }
        return (high-low + 1)/2 + 1;


        // a little bit optimized while loop solution
        let mut count: i32 = 0; 
        let mut temp_low: i32 = if low % 2 == 0 {low + 1} else {low};
        let mut temp_high: i32 = if high % 2 == 0 {high - 1} else {high};
        while temp_low <= temp_high{
            if temp_low == temp_high{
                if temp_low % 2 != 0{count+=1;}
                break;
            }
            count += 2;
            temp_high -= 2;
            temp_low += 2;
        }
        return count;
    }
}

