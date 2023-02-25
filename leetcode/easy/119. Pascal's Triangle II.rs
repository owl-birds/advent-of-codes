impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {return vec![1];}
        let mut row = vec![1,1];
        if row_index == 1 {return row;}

        let mut count_row = 1;

        while count_row < row_index{
            let prev_row = row.clone();
            row.push(1);
            // println!("{:?}", prev_row);
            let mut idx: usize = 0;
            let mut next_idx: usize = idx+1;

            while next_idx < prev_row.len(){
                let add = prev_row[idx] + prev_row[next_idx];
                row[idx+1] = add;
                idx += 1;
                next_idx += 1;
            }

            count_row += 1;    
        }

        return row;
    }
}
