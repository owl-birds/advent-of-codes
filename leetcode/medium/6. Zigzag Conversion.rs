impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut max_matrix: Vec<Vec<char>> = vec![];
        for r in 0..num_rows {
            let mut temp: Vec<char> = vec![];
            for c in 0..s.len() {
                temp.push('-');
            }
            max_matrix.push(temp);
        }
        // println!("{:?}", max_matrix);
        let mut is_diagonal: bool = false;
        // let mut is_up: bool = false;
        let mut r: usize = 0;
        let mut c: usize = 0;

        for letter in s.chars() {
            if !is_diagonal {
                // if !is_up {
                max_matrix[r][c] = letter;
                // }
                if r == (num_rows - 1) as usize {
                    is_diagonal = true;
                    // is_up = true;
                    r -= if r == 0 { 0 } else { 1 };
                    c += 1;
                    continue;
                }
                r += 1;
                continue;
            }
            max_matrix[r][c] = letter;
            if r == 0 {
                r += if r == (num_rows - 1) as usize { 0 } else { 1 };
                c += 1;
                is_diagonal = false;
                continue;
            }
            r -= 1;
            c += 1;
        }
        // println!("{:?}", max_matrix);

        let mut result: String = String::new();
        for row in max_matrix {
            for i in 0..row.len() {
                if row[i] != '-' {
                    result.push(row[i]);
                }
            }
        }

        result
    }
}
