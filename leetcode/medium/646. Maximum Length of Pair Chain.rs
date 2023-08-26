impl Solution {
    pub fn find_longest_chain(pairs: Vec<Vec<i32>>) -> i32 {
        let mut temp_pairs: Vec<Vec<i32>> = vec![];
        for pair in pairs {
            temp_pairs.push(pair.to_vec());
        }

        // if the same first num
        temp_pairs.sort_by(|a, b| a[1].cmp(&b[1]));
        // temp_pairs.sort_by(|a,b| {
        //     if a[0] != b[0] {
        //         a[0].cmp(&b[0])
        //     }else {
        //         a[1].cmp(&b[1])
        //     }
        // });
        // println!("pairs-->{:?}", temp_pairs);
        let mut longest: i32 = 1;
        let mut start: Vec<i32> = vec![];
        start.push(temp_pairs[0][0]);
        start.push(temp_pairs[0][1]);
        for i in 1..temp_pairs.len() {
            if start[1] < temp_pairs[i][0] {
                start[0] = temp_pairs[i][0];
                start[1] = temp_pairs[i][1];
                longest += 1;
            }
        }

        // let mut longest: i32 = 0;
        // for i in 0..temp_pairs.len() {
        //     let mut start: Vec<i32> = vec![];
        //     let mut temp: i32 = 1;
        //     start.push(temp_pairs[i][0]); // start with the first pair
        //     start.push(temp_pairs[i][1]); // start with the first pair
        //     // println!("start-->{:?}", start);
        //     for j in i+1..temp_pairs.len() {
        //         if start[1] < temp_pairs[j][0] {
        //             start[1] = temp_pairs[j][1];
        //             start[0] = temp_pairs[j][0];
        //             temp += 1;
        //         }
        //     }
        //     if temp > longest {
        //         longest = temp;
        //     }
        // }

        longest
    }
}
