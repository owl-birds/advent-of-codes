use std::collections::HashMap;
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        // use hashMap
        let mut row_map: HashMap<String, i32> = HashMap::new();
        let mut col_map: HashMap<String, i32> = HashMap::new();
        for i in 0..grid.len() {
            let mut row_string: String = String::from("");
            let mut col_string: String = String::from("");
            for j in 0..grid.len() {
                let mut row_digit = grid[i][j];
                let mut digit_row_string = String::from(row_digit.to_string());
                if j < grid.len() - 1 {
                    digit_row_string.push('-')
                }
                row_string.push_str(&digit_row_string[..]);
                let mut col_digit = grid[j][i];
                let mut digit_col_string = String::from(col_digit.to_string());
                if j < grid.len() - 1 {
                    digit_col_string.push('-')
                }
                col_string.push_str(&digit_col_string[..]);
            }
            let mut get_row = row_map.get_mut(&row_string);
            match get_row {
                Some(c_row) => *c_row += 1,
                None => {
                    row_map.insert(row_string, 1);
                }
            }
            let mut get_col = col_map.get_mut(&col_string);
            match get_col {
                Some(c_col) => *c_col += 1,
                None => {
                    col_map.insert(col_string, 1);
                }
            }
        }
        // println!("{:?}", row_map);
        // println!("{:?}", col_map);
        let mut count: i32 = 0;

        for (key, val_row) in row_map.iter() {
            if let Some(val_col) = col_map.get(key) {
                count += (val_row * val_col);
            }
        }

        count
    }
}
