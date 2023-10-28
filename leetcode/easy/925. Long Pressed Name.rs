impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        if name.len() > typed.len() {
            return false;
        }
        let mut temp_name: Vec<char> = name.chars().collect();
        let mut temp_typed: Vec<char> = typed.chars().collect();
        if temp_name[0] != temp_typed[0] {
            return false;
        }
        let mut i: usize = 0;
        let mut j: usize = 0;
        while i < name.len() && j < typed.len() {
            if temp_name[i] == temp_typed[j] {
                i += 1;
                j += 1;
                continue;
            }
            if temp_typed[j] == temp_name[i - 1] {
                j += 1;
                continue;
            }
            return false;
        }
        if i < temp_name.len() {
            return false;
        }
        // println!("i:{}\nj:{}",i,j);
        while j < temp_typed.len() {
            if temp_name[i - 1] == temp_typed[j] {
                j += 1;
            } else {
                return false;
            }
        }
        true
    }
}
