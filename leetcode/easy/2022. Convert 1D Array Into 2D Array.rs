impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        let mut two_d_arr: Vec<Vec<i32>> = vec![];
        if m * n != (original.len() as i32) {
            return two_d_arr;
        }
        let mut count: i32 = 0;
        let mut temp: Vec<i32> = vec![];
        let mut idx: usize = 0;
        while idx <= original.len() {
            if idx == original.len() {
                two_d_arr.push(temp.to_vec());
                temp = vec![];
                count = 0;
                break;
            }
            if count == n {
                two_d_arr.push(temp.to_vec());
                temp = vec![];
                count = 0;
            }
            temp.push(original[idx]);
            idx += 1;
            count += 1;
        }
        two_d_arr
    }
}
