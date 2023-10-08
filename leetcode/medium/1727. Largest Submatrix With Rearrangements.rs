impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let mut comulatives: Vec<Vec<i32>> = vec![];
        // for i in 0..matrix[0].len() {
        //     comulatives.push(vec![]);
        // }
        // println!("{:?}", comulatives);

        for i in 0..matrix[0].len() {
            let mut count: i32 = 0;
            let mut temp: Vec<i32> = vec![];
            for j in 0..matrix.len() {
                if matrix[j][i] == 0 {
                    count = 0;
                } else {
                    count += 1;
                }
                temp.push(count);
            }
            comulatives.push(temp);
        }
        // println!("{:?}", comulatives);

        let mut area: i32 = 0;

        for i in 0..comulatives[0].len() {
            let mut temp_row: Vec<i32> = vec![];
            for j in 0..comulatives.len() {
                temp_row.push(comulatives[j][i]);
            }
            temp_row.sort_by(|a, b| b.cmp(a));
            // println!("temp_row: {:?}", temp_row);
            let mut length: i32 = 0;
            let mut width: i32 = temp_row[0];
            for idx in 0..temp_row.len() {
                if temp_row[idx] == 0 {
                    break;
                }
                length += 1;
                if width > temp_row[idx] {
                    width = temp_row[idx];
                }
                let temp_area: i32 = width * length;
                if area < temp_area {
                    area = temp_area
                }
            }
        }

        area
    }
}
