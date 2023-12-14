use std::collections::HashMap;
impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        // let mut painted: Vec<Vec<i32>> = vec![];
        let mut min_idx: i32 = 0;
        let mut num_pos: HashMap<i32, Vec<usize>> = HashMap::new(); // [x, y] or [r, c]
        let mut row_count: Vec<i32> = vec![0; mat.len()];
        let mut col_count: Vec<i32> = vec![0; mat[0].len()];

        for r in 0..mat.len() {
            for c in 0..mat[0].len() {
                num_pos.insert(mat[r][c], vec![r, c]);
            }
        }
        // println!("{:?}", num_pos);
        for i in 0..arr.len() {
            if let Some(pos) = num_pos.get(&arr[i]) {
                row_count[pos[0]] += 1;
                col_count[pos[1]] += 1;
                if row_count[pos[0]] == mat[0].len() as i32 || col_count[pos[1]] == mat.len() as i32
                {
                    min_idx = i as i32;
                    break;
                }
            }
        }

        min_idx
    }
    // pub fn
}
