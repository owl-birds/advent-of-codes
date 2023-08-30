impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut temp: Vec<i32> = nums.to_vec();
        let mut count: i32 = 0;

        for i in 1..temp.len() {
            if temp[i] > temp[i - 1] {
                continue;
            }
            if temp[i] < temp[i - 1] {
                count += (temp[i - 1] - temp[i]) + 1;
                temp[i] = temp[i - 1] + 1;
                continue;
            }
            // temp[i] == temp[i-1]
            count += 1;
            temp[i] += 1;
        }

        count
    }
}
