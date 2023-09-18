impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // 1 : soldiers
        // 0 : civilians
        let row: usize = mat.len();
        // let mut row_soldier: [[i32; 2]; 4] = [[0,0]; 4];
        let mut row_soldier: Vec<Vec<i32>> = vec![vec![0, 0]; row];
        // [idx, count_soldiers]
        for i in 0..mat.len() {
            let mut idx: usize = 0;
            let mut count: i32 = 0;
            while idx < mat[i].len() && mat[i][idx] == 1 {
                count += 1;
                idx += 1;
            }
            row_soldier[i][0] = i as i32;
            row_soldier[i][1] = count;
        }
        row_soldier.sort_by(|a, b| {
            if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut result: Vec<i32> = vec![];
        // println!("{:?}", &row_soldier[..k as usize]);
        for i in 0..k as usize {
            result.push(row_soldier[i][0]);
        }

        result
    }
}
