impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut alps: [char; 26] = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];

        let mut result: String = String::from("");

        let mut temp: usize = column_number as usize;
        let mut idxs: Vec<usize> = vec![];

        while temp > 0 {
            temp -= 1;
            let remainder = temp % 26;
            // println!("-->{}", remainder);
            idxs.push(remainder);
            temp = temp / 26;
        }
        // println!("{:?}", idxs);
        let mut i: usize = idxs.len();

        while i >= 1 {
            i -= 1;
            result.push(alps[idxs[i]]);
        }

        result
    }
}
