impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut start_banana: i32 = 1;
        let mut end_banana: i32 = *piles.iter().max().unwrap();
        while start_banana < end_banana{
            let mid: i32 = (start_banana+end_banana)/2;
            if Self::can_eat(mid, &piles, h){
                end_banana = mid;
                continue;
            }
            start_banana = mid + 1;
        }
        return start_banana;
    }
    pub fn can_eat(eat: i32, piles: &Vec<i32>, hour: i32) -> bool{
        let mut count_hour: i32 = 0;
        for pile in piles{
            if pile % eat == 0{
                count_hour += (pile/eat);
            }else{
                count_hour += (pile/eat) + 1;
            }
            if count_hour > hour{
                return false;
            }
        }
        return true;
    }
}