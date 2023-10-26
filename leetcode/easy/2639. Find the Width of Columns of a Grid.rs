impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for c in 0..grid[0].len() {
            let mut longest: i32 = i32::MIN;
            for r in 0..grid.len() {
                let curr_dig_length: i32 = Self::count_digit_length(grid[r][c]);
                if curr_dig_length > longest {
                    longest = curr_dig_length;
                }
            }
            result.push(longest);
        }

        result
    }
    pub fn count_digit_length(num: i32) -> i32 {
        if num == 0 {
            return 1;
        }
        let mut temp: i32 = if num < 0 { num * -1 } else { num };
        let mut count: i32 = if num < 0 { 1 } else { 0 };

        while temp > 0 {
            count += 1;
            temp /= 10;
        }

        count
    }
}
