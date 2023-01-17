impl Solution {
    pub fn min_flips_mono_incr(s: String) -> i32 {
        let mut one_flips: i32 = 0;
        let mut count_flips: i32 = 0;
        for c in s.chars(){
            if c == '0'{
                if one_flips == 0 {
                    continue;
                }
                count_flips += 1
            }
            if c == '1'{
                one_flips += 1;
            }
            if count_flips > one_flips{
                count_flips = one_flips;
            }
        }
        return count_flips;
    }
}