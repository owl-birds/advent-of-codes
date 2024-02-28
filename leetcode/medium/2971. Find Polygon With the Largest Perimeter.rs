impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i64 {
        let mut result: i64 = -1;
        let mut pref_sum: Vec<i64> = vec![];
        let mut temp: Vec<i32> = nums.to_vec();
        temp.sort_by(|a, b| a.cmp(&b));
        let mut sum: i64 = 0;
        for i in 0..temp.len() {
            sum += temp[i] as i64;
            pref_sum.push(sum);
        }
        for i in 2..pref_sum.len() {
            if pref_sum[i - 1] > temp[i] as i64 {
                result = pref_sum[i];
            }
        }
        result
    }
}
