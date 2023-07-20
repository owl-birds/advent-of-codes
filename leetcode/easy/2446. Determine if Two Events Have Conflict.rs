impl Solution {
    pub fn have_conflict(event1: Vec<String>, event2: Vec<String>) -> bool {
        let mut event1_parsed: Vec<Vec<i32>> = vec![];
        for i in 0..event1.len() {
            event1_parsed.push(Self::parsing_time_string_by(&event1[i], ':'));
        }
        let mut event1_min: Vec<i32> = vec![];
        for i in 0..event1_parsed.len() {
            event1_min.push(event1_parsed[i][1] + (event1_parsed[i][0] * 60));
        }

        let mut event2_parsed: Vec<Vec<i32>> = vec![];
        for i in 0..event2.len() {
            event2_parsed.push(Self::parsing_time_string_by(&event2[i], ':'));
        }
        let mut event2_min: Vec<i32> = vec![];
        for i in 0..event2_parsed.len() {
            event2_min.push(event2_parsed[i][1] + (event2_parsed[i][0] * 60));
        }

        if event2_min[0] <= event1_min[0] && event1_min[0] <= event2_min[1] {
            return true;
        }
        if event2_min[0] <= event1_min[1] && event1_min[1] <= event2_min[1] {
            return true;
        }
        if event1_min[0] <= event2_min[1] && event2_min[1] <= event1_min[1] {
            return true;
        }
        if event1_min[0] <= event2_min[0] && event2_min[0] <= event1_min[1] {
            return true;
        }

        println!("event1:{:?}", event1_parsed);
        println!("event2:{:?}", event2_parsed);
        println!("event1:{:?}", event1_min);
        println!("event2:{:?}", event2_min);

        println!(
            "{:?}=>{:?}",
            event1[0],
            Self::parsing_time_string_by(&event1[0], ':')
        );
        println!(
            "{:?}=>{:?}",
            event2[0],
            Self::parsing_time_string_by(&event2[0], ':')
        );
        println!(
            "{:?}=>{:?}",
            String::from("09:23:60"),
            Self::parsing_time_string_by(&String::from("09:23:60"), ':')
        );
        println!(
            "{:?}=>{:?}",
            String::from("09-23-60"),
            Self::parsing_time_string_by(&String::from("09-23-60"), '-')
        );

        false
    }
    pub fn parsing_time_string_by(time: &String, by: char) -> Vec<i32> {
        // [hour, minute] // only accepts => hours:minutes
        let mut time_num: Vec<i32> = vec![];
        let mut time_char: Vec<char> = time.chars().collect();

        let mut temp_num: i32 = 0;
        let mut multiplication: i32 = 1;
        let mut idx: i32 = time_char.len() as i32 - 1;

        while idx >= 0 {
            if time_char[idx as usize] == by {
                time_num.insert(0, temp_num);
                temp_num = 0;
                multiplication = 1;
                idx -= 1;
                continue;
            }
            temp_num += (time_char[idx as usize].to_digit(10).unwrap() as i32 * multiplication);
            multiplication *= 10;
            idx -= 1;
        }
        time_num.insert(0, temp_num);

        // while idx >= 0 {
        //     temp_num += (time_char[idx as usize].to_digit(10).unwrap() as i32 * multiplication);
        //     multiplication *= 10;
        //     idx -= 1;
        // }
        // time_num.push(temp_num);

        // let mut temp_num: i32 = 0;
        // let mut multiplication: i32 = 1;
        // let mut idx: i32 = time_char.len() as i32 - 1;

        // while time_char[idx as usize] != ':' {
        //     temp_num += (time_char[idx as usize].to_digit(10).unwrap() as i32 * multiplication);
        //     multiplication *= 10;
        //     idx -= 1;
        // }
        // time_num.push(temp_num);

        time_num
    }
}
