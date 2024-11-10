impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        // let mut temp: i32 = left.len() as i32 + right.len() as i32;
        let mut time: i32 = 0;
        for i in 0..left.len() {
            if left[i] > time {
                time = left[i]
            }
        }
        for i in 0..right.len() {
            if i32::abs(right[i] - n) > time {
                time = i32::abs(right[i] - n)
            }
        }
        // let mut mut_left: Vec<i32> = left.to_vec();
        // let mut mut_right: Vec<i32> = right.to_vec();
        // // MAYBE TLE ? :: BRUTE FORCE
        // println!("{:?}", mut_left);
        // println!("{:?}", mut_right);
        // println!("---------");
        // while temp > 0 {
        //     // ants moved
        //     /// mut_left
        //     for i in 0..mut_left.len() {
        //         mut_left[i] -= 1;
        //     }
        //     /// mut_right
        //     for i in 0..mut_right.len() {
        //         mut_right[i] += 1;
        //     }
        //     // checking the ants that met at the same point
        //     for i in 0..mut_left.len() {
        //         for j in 0..mut_right.len() {

        //         }
        //     }

        //     // checking the fallen ant and remove it from the group
        //     /// mut_left
        //     let mut idxs: Vec<usize> = vec![];
        //     for i in 0..mut_left.len() {
        //         if mut_left[i] <= 0 {idxs.push(i)}
        //     }
        //     temp -= idxs.len() as i32;
        //     for i in idxs {
        //         mut_left.remove(i);
        //     }
        //     /// mut_right
        //     let mut idxs: Vec<usize> = vec![];
        //     for i in 0..mut_right.len() {
        //         if mut_right[i] >= n {idxs.push(i)}
        //     }
        //     temp -= idxs.len() as i32;
        //     for i in idxs {
        //         mut_right.remove(i);
        //     }

        //     time += 1;
        //     println!("{:?}", mut_left);
        //     println!("{:?}", mut_right);
        //     println!("---------");
        //     // break point ::: dev only
        //     temp = 0;
        // }

        // println!("ants:{}", temp);
        time
    }
}
