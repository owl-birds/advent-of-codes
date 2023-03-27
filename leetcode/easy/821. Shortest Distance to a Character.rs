impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let mut c_indexes: Vec<i32> = vec![];
        let mut result: Vec<i32> = vec![];

        for (i, ch) in s.chars().enumerate(){
            if ch == c {c_indexes.push(i as i32);}
        }
        // println!("{:?}", c_indexes);

        for (s_i, ch) in s.chars().enumerate(){
            let mut min_abs_diff: i32 = (s_i as i32 - &c_indexes[0]).abs();
            // println!("{}", min_abs_diff);
            for idx in &c_indexes{
                let curr_diff: i32 = (s_i as i32 - idx).abs();
                if min_abs_diff > curr_diff{
                    min_abs_diff = curr_diff;
                }
            }
            result.push(min_abs_diff);
        }

        result
    }
}
