impl Solution {
    pub fn can_make_arithmetic_progression(arr: Vec<i32>) -> bool {
       let mut temp_arr: Vec<i32> = arr.clone();
       temp_arr.sort();
       let diff: i32 = i32::abs(temp_arr[1]-temp_arr[0]);

       for i in 2..temp_arr.len(){
           if diff != i32::abs(temp_arr[i]-temp_arr[i-1]){return false}
       }
       true
    }
}