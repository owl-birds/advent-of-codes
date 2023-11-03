impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut t_pointer: usize = 0;
        let mut result: Vec<String> = vec![];

        for num in 1..n + 1 {
            if t_pointer >= target.len() {
                break;
            }
            if num == target[t_pointer] {
                result.push(String::from("Push"));
                t_pointer += 1;
                continue;
            }
            result.push(String::from("Push"));
            result.push(String::from("Pop"));
        }

        result
    }
}
