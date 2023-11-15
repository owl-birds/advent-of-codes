impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut temp: Vec<i32> = arr.to_vec();
        // let mut max: i32 = 1;

        temp.sort_by(|a, b| a.cmp(b));
        // println!("{:?}", temp);
        temp[0] = 1;
        for i in 1..temp.len() {
            if temp[i] == (temp[i - 1] + 1) || temp[i] == temp[i - 1] {
                continue;
            }
            temp[i] = temp[i - 1] + 1;
        }
        // println!("{:?}", temp);

        // max
        temp[temp.len() - 1]
    }
}
