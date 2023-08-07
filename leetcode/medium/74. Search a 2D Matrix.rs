impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for row in matrix {
            let mut left: usize = 0;
            let mut right: usize = row.len() - 1;
            // if left == right {
            //     if row[left] == target {
            //         return true;
            //     }
            //     continue;
            // }
            while left < right {
                let mid: usize = (left + right) / 2;
                if row[mid] == target {
                    return true;
                }
                if row[mid] > target {
                    right = mid;
                    continue;
                }
                left = mid + 1;
            }
            if row[left] == target {
                return true;
            }
        }

        false
    }
}
