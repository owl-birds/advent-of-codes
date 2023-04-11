impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut new_string_vec: Vec<char> = vec![];

        // im BAD SO ITS OKAY TO THIS
        let mut chars: Vec<char> = vec![];
        for c in s.chars(){
            chars.push(c);
        }

        let mut idx: i32 = 0;
        while idx < chars.len() as i32{
            if chars[idx as usize] == '*'{
                let mut count_stars: i32 = 0;        
                while idx < chars.len() as i32 && chars[idx as usize] == '*'{
                    count_stars += 1;
                    idx += 1;
                }
                while count_stars > 0{
                    new_string_vec.pop();
                    count_stars -= 1;
                }
                continue;
            }

            new_string_vec.push(chars[idx as usize]);
            idx += 1;
        }
        // println!("{:?}", chars);
        println!("{:?}", new_string_vec);

        let mut new_string: String = String::from("");
        for c in new_string_vec{
            new_string.push(c);
        }
        new_string
        // new_string_vec.iter().collect()
    }
}
