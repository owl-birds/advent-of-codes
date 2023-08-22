impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let mut alps: [char; 26] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        let s_vec: Vec<char> = s.chars().collect();
        let min: char = s_vec[1];
        let min: u32 = min.to_digit(10).unwrap();
        let max: char = s_vec[4];
        let max: u32 = max.to_digit(10).unwrap();
        let start: char = s_vec[0];
        let mut start_idx: usize = 0;
        let end: char = s_vec[3];
        let mut end_idx: usize = 0;
        for i in 0..alps.len() {
            if alps[i] == start {
                start_idx = i;
            }
            if alps[i] == end {
                end_idx = i;
            }
        }
        // println!("{}->{}\n{}->{}", min, max, start_idx, end_idx);
        let mut result: Vec<String> = vec![];
        for i in start_idx..end_idx + 1 {
            for num in min..max + 1 {
                let mut cell: String = String::from("");
                cell.push(alps[i]);
                let num_char: Vec<char> = num.to_string().chars().collect();
                for n in num_char {
                    cell.push(n);
                }
                result.push(cell);
            }
        }

        result
    }
}
