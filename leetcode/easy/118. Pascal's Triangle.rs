impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows == 0 {return vec![];} 
        let mut tri: Vec<Vec<i32>> = vec![vec![1], vec![1,1]];
        if num_rows == 1 {return vec![vec![1]];}
        if num_rows == 2 {return tri;}

        let mut curr_row = 2;

        while curr_row < num_rows{
            let mut prev_row = &tri[(curr_row-1) as usize];
            let mut new_row: Vec<i32> = vec![1];
            let mut idx: usize = 0;
            let mut next_idx: usize = 1;
            println!("{:?}", prev_row);
            while next_idx <= (prev_row.len()-1){
                let add = prev_row[idx] + prev_row[next_idx];
                new_row.push(add);
                idx += 1;
                next_idx += 1;
            }
            // [1,...,1]
            new_row.push(1);
            //
            tri.push(new_row);
            curr_row += 1;
        }

        return tri;
    }
}
