impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut curr_mass: i64 = mass as i64;
        let mut sorted_ast: Vec<i32> = asteroids.to_vec();
        sorted_ast.sort_by(|a, b| a.cmp(b));
        // println!("{:?}", sorted_ast);
        for i in 0..sorted_ast.len() {
            if curr_mass < sorted_ast[i] as i64 {
                return false;
            }
            if curr_mass >= 100000 {
                return true;
            }
            curr_mass += sorted_ast[i] as i64;
            // println!("curr_mass:{}", curr_mass);
        }

        true
    }
}
