impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        // let mut temp_arr1: Vec<Vec<i32>> = vec![];
        // for arr in arrays {
        //     temp_arr1.push(arr.to_vec());
        // }
        // temp_arr1.sort_by(|a, b| {
        //     if a[0] != b[0] {return a[0].cmp(&b[0])}
        //     a[a.len()-1].cmp(&b[b.len()-1])
        // });
        // println!("1:{:?}", temp_arr1);
        let mut min_curr: i32 = arrays[0][0];
        let mut max_curr: i32 = arrays[0][arrays[0].len() - 1];
        let mut max_dist: i32 = 0;
        for i in 1..arrays.len() {
            let curr_dist: i32 = i32::abs(arrays[i][arrays[i].len() - 1] - min_curr);
            if curr_dist > max_dist {
                max_dist = curr_dist;
            }
            let curr_dist: i32 = i32::abs(arrays[i][0] - max_curr);
            if curr_dist > max_dist {
                max_dist = curr_dist;
            }
            if arrays[i][0] < min_curr {
                min_curr = arrays[i][0];
            }
            if arrays[i][arrays[i].len() - 1] > max_curr {
                max_curr = arrays[i][arrays[i].len() - 1];
            }
        }
        max_dist
    }
}
