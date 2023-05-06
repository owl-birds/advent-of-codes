impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut count_changes: i32 = 0;
        let mut min_changes: i32 = 0;
        let blocks_char: Vec<char> = blocks.chars().collect();

        // count the first white block in the k range
        let mut idx: usize = 0;

        while idx < k as usize{
            if idx == k as usize{break}
            if blocks_char[idx] == 'W'{count_changes+=1}
            idx += 1;
        }
        min_changes = count_changes;
        // println!("{}, count {}", idx, min_changes);
        // println!("{:?}", blocks_char);

        while idx < blocks_char.len(){
            if min_changes == 0{return min_changes}
            if blocks_char[idx] == 'W'{
                count_changes += 1;
            }
            if blocks_char[idx-k as usize] == 'W'{
                count_changes -= 1;
            }
            if min_changes > count_changes{
                min_changes = count_changes;
            }
            idx += 1;
        }

        min_changes
    }
}