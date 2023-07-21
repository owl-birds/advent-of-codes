impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        for i in 0..intervals.len() {
            result.push(intervals[i].to_vec());
        }
        // i have to sort it OK, im lazy ok, im to lazy to think a better slution, at least it WORKS HAAHHAAHA
        result.sort_by(|a, b| a[0].cmp(&b[0]));

        // println!("{:?}", result);
        let mut idx: usize = 0;
        while idx < result.len() {
            let mut temp_idx: usize = idx + 1;
            while temp_idx < result.len() {
                if temp_idx == idx {
                    temp_idx += 1;
                    continue;
                }
                if result[idx][0] <= result[temp_idx][0] && result[temp_idx][0] <= result[idx][1] {
                    if result[temp_idx][1] > result[idx][1] {
                        result[idx][1] = result[temp_idx][1];
                    }
                    result.remove(temp_idx);
                    continue;
                } else if result[idx][0] <= result[temp_idx][1]
                    && result[temp_idx][1] <= result[idx][1]
                {
                    if result[temp_idx][0] < result[idx][0] {
                        result[idx][0] = result[temp_idx][0];
                    }
                    result.remove(temp_idx);
                    continue;
                }
                temp_idx += 1;
            }
            // println!("=>{:?}", result);
            idx += 1;
        }
        // println!("{:?}", result);

        result
    }
}
