impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        if asteroids.len() <= 1 {
            return asteroids;
        }
        let mut result: Vec<i32> = asteroids.to_vec();
        let mut is_right: bool = asteroids[0] >= 0;
        let mut idx: usize = 0;
        while idx < result.len() && result[idx] < 0 {
            idx += 1;
        }
        if idx == 0 {
            idx += 1
        }
        println!("start_idx:{}", idx);
        while idx < result.len() {
            if result[idx] < 0 {
                let ast_size: i32 = i32::abs(result[idx]);
                let mut ast_idx: usize = idx;
                // println!("HELOO:{}", idx);
                while idx >= 1 && result[idx - 1] > 0 {
                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                    // println!("-->{}vs{}", result[idx], ast_size);
                    if ast_size == result[idx] {
                        result.remove(idx);
                        ast_idx -= 1;
                        result.remove(ast_idx);
                        idx -= 1;
                        break;
                    }
                    if ast_size < result[idx] {
                        result.remove(ast_idx);
                        break;
                    }
                    if ast_size > result[idx] {
                        result.remove(idx);
                        ast_idx -= 1;
                    }
                }
            }
            // println!("idx:{}",idx);
            idx += 1;
        }
        // println!("{}=>is_right:{}", asteroids[0], is_right);

        result
    }
}
