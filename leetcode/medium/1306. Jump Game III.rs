use std::collections::HashSet;
impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited: HashSet<usize> = HashSet::new();
        let mut is_found: [bool; 1] = [false];
        Self::jump(&arr, start as usize, &mut visited, &mut is_found);

        is_found[0]
    }
    pub fn jump(
        arr: &Vec<i32>,
        curr_pos: usize,
        visited: &mut HashSet<usize>,
        is_found: &mut [bool; 1],
    ) {
        // BASE CASE
        // println!("pos:{} - num:{}", curr_pos, arr[curr_pos]);
        if is_found[0] {
            return;
        }
        if arr[curr_pos] == 0 {
            is_found[0] = true;
            return;
        }
        if visited.contains(&curr_pos) {
            return;
        }
        visited.insert(curr_pos);

        // recurse
        if (curr_pos + arr[curr_pos] as usize) < arr.len() {
            Self::jump(arr, curr_pos + arr[curr_pos] as usize, visited, is_found);
        }
        if curr_pos >= (arr[curr_pos] as usize) {
            Self::jump(arr, curr_pos - arr[curr_pos] as usize, visited, is_found);
        }
        // END
        is_found[0] = is_found[0] || false;
    }
}
