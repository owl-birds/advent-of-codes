use std::collections::HashMap;
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        // idk is the solution below is any good
        let nums: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut nums_table: HashMap<char, u8> = HashMap::new();
        for num in 0..10 as usize {
            nums_table.insert(nums[num], num as u8);
        }
        // println!("{:?}", nums_table);
        // idk is the solution above is any good

        // better solution
        let mut length_of_decoded: i64 = 0;
        for c in s.chars() {
            if let Some(num) = nums_table.get(&c) {
                length_of_decoded *= *num as i64;
                continue;
            }
            length_of_decoded += 1;
        }
        let mut idx: usize = s.len();
        let mut temp_s: Vec<char> = s.chars().collect();
        let mut temp_k: i64 = k as i64;
        let mut result: char = '0';
        // println!("k : {} -- idx : {} -- length : {}", temp_k, idx, length_of_decoded);
        while idx >= 1 {
            idx -= 1;
            // println!("k : {} -- idx : {} -- length : {}", temp_k, idx, length_of_decoded);
            if let Some(num) = nums_table.get(&temp_s[idx]) {
                // println!("{}-{}",  temp_s[idx], num);
                length_of_decoded /= *num as i64;
                // if length_of_decoded == 0 {continue}
                temp_k = temp_k % length_of_decoded;
                continue;
            }
            if temp_k == 0 || length_of_decoded == temp_k {
                result = temp_s[idx];
                break;
            }
            length_of_decoded -= 1;
        }
        // println!("k : {} -- idx : {} -- length : {}", temp_k, idx, length_of_decoded);
        result.to_string()
        // better solution

        // let mut decoded_string: Vec<char> = vec![];

        // for c in s.chars() {
        //     if decoded_string.len() >= k as usize {break}
        //     if let Some(num) = nums_table.get(&c) {
        //         // println!("{}", num);
        //         let mut temp: u8 = num-1;
        //         let limit: usize = decoded_string.len();
        //         while temp >= 1 {
        //             temp -= 1;
        //             for i in 0..limit {
        //                 decoded_string.push(decoded_string[i]);
        //             }
        //         }
        //         continue
        //     }
        //     decoded_string.push(c);
        // }
        // // println!("{:?}", decoded_string);

        // // println!("{} --- {}", decoded_string.len(), length_of_decoded);

        // // "TEMP".to_string()
        // decoded_string[(k-1) as usize].to_string()
    }
}

// 1. Reverse

//     Initialize decodedLength to 0, representing the total length of the decoded string.
//     Iterate through each character in the input string.
//         If the character is a digit, update decodedLength by multiplying it with the digit.
//         If the character is a letter, increment decodedLength.
//     Traverse the input string in reverse order.
//     For each character:
//         If it's a digit adjust decodedLength and k accordingly.
//         If it's a letter Check if it's the kth character or if k is 0.
//             If yes, return the character as a string.
//             If not, decrement decodedLength.

// Complexity

//     Time complexity: O(N)O(N)O(N)
//     Since we are iterating over the string twice, one time to calculate lengths for each character in the encoded string and another time to get kth character then time complexity is 2*N which is O(N).
//     Space complexity: O(1)O(1)O(1)
//     Since we are only storing couple of constant variables.
