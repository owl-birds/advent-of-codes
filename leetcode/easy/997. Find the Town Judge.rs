use std::collections::HashMap;
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut give: HashMap<i32, i32> = HashMap::new();
        let mut receive: HashMap<i32, i32> = HashMap::new();
        for people in 1..n + 1 {
            give.insert(people, 0);
            receive.insert(people, 0);
        }
        for i in 0..trust.len() {
            if let Some(freq) = give.get_mut(&trust[i][0]) {
                *freq += 1;
            }
            if let Some(freq) = receive.get_mut(&trust[i][1]) {
                *freq += 1;
            }
        }
        // println!("{:?}\n{:?}", give, receive);
        let mut who: i32 = -1;
        for (people, trust_given) in give.iter() {
            if *trust_given == 0 {
                if let Some(trust_received) = receive.get(&people) {
                    if *trust_received == (n - 1) {
                        who = *people;
                        break;
                    }
                }
            }
        }
        who
    }
}
