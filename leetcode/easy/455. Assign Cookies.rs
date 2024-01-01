impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut temp_g: Vec<i32> = g.to_vec();
        let mut temp_s: Vec<i32> = s.to_vec();
        temp_g.sort_by(|a, b| a.cmp(&b));
        temp_s.sort_by(|a, b| a.cmp(&b));
        let mut count: i32 = 0;
        let mut g_idx: usize = 0;
        // println!("g:{:?}\ns:{:?}", temp_g, temp_s);
        for i in 0..temp_s.len() {
            if g_idx >= temp_g.len() {
                break;
            }
            if temp_s[i] < temp_g[g_idx] {
                continue;
            };
            count += 1;
            g_idx += 1;
        }
        count
    }
}
