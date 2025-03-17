use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let mut correct: Vec<String> = vec![];
        // let mut words_map: HashMap<String, HashSet<String>> = HashMap::new();
        let mut words_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut vowels: HashSet<char> = HashSet::from(['a', 'i', 'u', 'e', 'o']);
        let mut memoize: HashMap<String, String> = HashMap::new(); // reducing redundant
        for i in 0..wordlist.len() {
            // let mut temp: Vec<char> = vec![];
            let mut word_str: String = String::new();
            for c in wordlist[i].to_lowercase().chars() {
                if vowels.contains(&c) {
                    word_str.push('_');
                    continue;
                }
                word_str.push(c);
            }
            // println!("{:?}", word_str);
            if let Some(words) = words_map.get_mut(&word_str.to_lowercase()) {
                // words.insert(wordlist[i].to_string());
                words.push(wordlist[i].to_string());
            } else {
                // words_map.insert(wordlist[i].to_lowercase(), HashSet::from([wordlist[i].to_string()]));
                words_map.insert(word_str.to_lowercase(), vec![wordlist[i].to_string()]);
            }
        }
        // println!("{:?}", words_map);
        // println!("{:?}\n---", vowels);
        for i in 0..queries.len() {
            // let mut temp: Vec<char> = vec![];
            let mut word_q: String = String::new();
            for c in queries[i].to_lowercase().chars() {
                if vowels.contains(&c) {
                    word_q.push('_');
                    continue;
                }
                word_q.push(c);
            }
            // println!("{:?}", word_q);
            if let Some(words) = words_map.get(&word_q) {
                let mut is_found: bool = false;
                for j in 0..words.len() {
                    if words[j] == queries[i] {
                        is_found = true;
                        break;
                    }
                }
                if is_found {
                    correct.push(queries[i].to_string());
                    continue;
                }
                let mut idx: i32 = -1;
                for j in 0..words.len() {
                    if words[j].to_lowercase() == queries[i].to_lowercase() {
                        idx = j as i32;
                        break;
                    }
                }
                if idx != -1 {
                    correct.push(words[idx as usize].to_string());
                    continue;
                }
                correct.push(words[0].to_string());
            } else {
                correct.push("".to_string());
            }
        }

        // TLE
        // for i in 0..queries.len() {
        //     if let Some(word) = memoize.get(&queries[i]) {
        //         correct.push(word.to_string());
        //         continue;
        //     }
        //     if let Some(words) = words_map.get(&queries[i].to_lowercase()) {
        //         let mut is_exist: bool = false;
        //         for j in 0..words.len() {
        //             if words[j] == queries[i] {
        //                 is_exist = true;
        //                 break;
        //             }
        //         }
        //         if is_exist {
        //             correct.push(queries[i].to_string());
        //             memoize.insert(queries[i].to_string(), queries[i].to_string());
        //         } else {
        //             correct.push(words[0].to_string());
        //             memoize.insert(queries[i].to_string(), words[0].to_string());
        //         }
        //         continue;
        //     } else {
        //         let mut is_match: bool = true;
        //         for j in 0..wordlist.len() {
        //             if wordlist[j].len() != queries[i].len() {
        //                 is_match = false;
        //                 continue;
        //             }
        //             let temp_q: Vec<char> = queries[i].to_lowercase().chars().collect();
        //             let temp_w: Vec<char> = wordlist[j].to_lowercase().chars().collect();
        //             // println!("q{:?}\nw{:?}\n------", temp_q, temp_w);
        //             is_match = true;
        //             for k in 0..temp_q.len() {
        //                 if vowels.contains(&temp_q[k]) && vowels.contains(&temp_w[k]) {
        //                     continue;
        //                 }
        //                 if temp_q[k] != temp_w[k] {
        //                     // println!("{}-{}", temp_q[k], temp_w[k]);
        //                     is_match = false;
        //                     break;
        //                 }
        //             }
        //             if is_match {
        //                 correct.push(wordlist[j].to_string());
        //                 memoize.insert(queries[i].to_string(), wordlist[j].to_string());
        //                 break;
        //             } else {
        //                 continue;
        //             }
        //         }
        //         // println!("{:?}-{}", queries[i], is_match);
        //         if !is_match {
        //             correct.push("".to_string());
        //         }
        //         // correct.push("9999".to_string());
        //     }
        // }
        // TLE
        correct
    }
}
