impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut t_a: i32 = a;
        let mut t_b: i32 = b;
        let mut t_c: i32 = c;
        let mut score: i32 = 0;
        while t_a != 0 && t_b != 0 || t_a != 0 && t_c != 0 || t_b != 0 && t_c != 0 {
            // println!("a:{},b:{},c:{}-->{}", t_a,t_b,t_c,score);
            score += 1;
            if t_a >= t_b && t_a >= t_c {
                if t_b >= t_c {
                    t_a -= 1;
                    t_b -= 1;
                } else if t_b < t_c {
                    t_a -= 1;
                    t_c -= 1;
                }
            } else if t_b >= t_a && t_b >= t_c {
                if t_a >= t_c {
                    t_b -= 1;
                    t_a -= 1;
                } else if t_a < t_c {
                    t_b -= 1;
                    t_c -= 1;
                }
            } else if t_c >= t_a && t_c >= t_b {
                if t_a >= t_b {
                    t_c -= 1;
                    t_a -= 1;
                } else if t_a < t_b {
                    t_c -= 1;
                    t_b -= 1;
                }
            }
        }
        score
    }
}
