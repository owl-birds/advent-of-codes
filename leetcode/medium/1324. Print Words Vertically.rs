impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        let mut s_chars: Vec<char> = s.chars().collect();

        let mut words: Vec<Vec<char>> = Self::split_string_by(s_chars, ' ');
        let col: usize = words.len();
        let mut row: usize = 0;
        for i in 0..words.len() {
            if words[i].len() > row {
                row = words[i].len();
            }
        }
        let mut words_matrix: Vec<Vec<char>> = vec![];
        for r in 0..row {
            let mut temp_row: Vec<char> = vec![];
            for c in 0..col {
                temp_row.push(' ');
            }
            words_matrix.push(temp_row);
        }
        // println!("{:?}", words);
        println!("row:{}\ncol:{}", row, col);
        let mut col_idx: usize = 0;
        for word in words {
            let mut row_idx: usize = 0;
            while row_idx < word.len() {
                words_matrix[row_idx][col_idx] = word[row_idx];
                row_idx += 1;
            }
            col_idx += 1;
        }
        // println!("{:?}", words_matrix);
        for row in words_matrix {
            let mut temp_str: String = String::new();
            let mut limit_length: usize = row.len() - 1; // cause no trailing spaces
            while row[limit_length] == ' ' {
                limit_length -= 1;
            }
            for i in 0..limit_length + 1 {
                temp_str.push(row[i]);
            }
            result.push(temp_str);
        }
        result
    }
    pub fn split_string_by(s_chars: Vec<char>, by: char) -> Vec<Vec<char>> {
        let mut result: Vec<Vec<char>> = vec![];
        let mut idx: usize = 0;

        while idx < s_chars.len() {
            let mut temp_word: Vec<char> = vec![];
            while idx < s_chars.len() && s_chars[idx] != by {
                temp_word.push(s_chars[idx]);
                idx += 1;
            }
            result.push(temp_word);
            idx += 1;
        }

        result
    }
}
