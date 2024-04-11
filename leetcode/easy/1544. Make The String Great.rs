use std::collections::HashMap;
use std::collections::VecDeque;
impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: VecDeque<char> = VecDeque::new();
        let mut temp: Vec<char> = s.chars().collect();
        let mut upp_low: HashMap<char, char> = HashMap::from([
            ('A', 'a'),
            ('B', 'b'),
            ('C', 'c'),
            ('D', 'd'),
            ('E', 'e'),
            ('F', 'f'),
            ('G', 'g'),
            ('H', 'h'),
            ('I', 'i'),
            ('J', 'j'),
            ('K', 'k'),
            ('L', 'l'),
            ('M', 'm'),
            ('N', 'n'),
            ('O', 'o'),
            ('P', 'p'),
            ('Q', 'q'),
            ('R', 'r'),
            ('S', 's'),
            ('T', 't'),
            ('U', 'u'),
            ('V', 'v'),
            ('W', 'w'),
            ('X', 'x'),
            ('Y', 'y'),
            ('Z', 'z'),
        ]);
        let mut low_upp: HashMap<char, char> = HashMap::new();
        for (upper, lower) in upp_low.iter() {
            low_upp.insert(*lower, *upper);
        }
        // println!("{:?}\n{:?}", upp_low, low_upp);
        let mut idx: usize = 0;

        while idx < temp.len() {
            // check top and check next char, if it is lower or upper skip the vec or remove the top
            let back: Option<&char> = stack.back();
            // checking if the current is upper or lower, and then checing the prev or the next char
            let is_upper = upp_low.contains_key(&temp[idx]);
            // println!("letter:{} -- is_upper:{}", temp[idx],is_upper);

            match back {
                Some(letter) => {
                    if is_upper {
                        let low_letter: char = *upp_low.get(&temp[idx]).unwrap();
                        if *letter == low_letter {
                            stack.pop_back();
                            idx += 1;
                            continue;
                        }
                    } else {
                        let upp_letter: char = *low_upp.get(&temp[idx]).unwrap();
                        if *letter == upp_letter {
                            stack.pop_back();
                            idx += 1;
                            continue;
                        }
                    }
                    stack.push_back(temp[idx]);
                    idx += 1;
                }
                None => {
                    stack.push_back(temp[idx]);
                    idx += 1;
                }
            }
        }

        let mut ans: String = String::new();
        for c in stack.iter() {
            ans.push(*c);
        }
        ans
    }
}
