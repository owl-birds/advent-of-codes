impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        // check every bank/row
        let mut laser_count_row: Vec<i32> = vec![];
        for b in bank{
            let mut count_laser: i32 = 0;
            for floor in b.chars(){
                if floor == '1'{
                    count_laser += 1;
                }
            }
            if count_laser > 0{laser_count_row.push(count_laser)}
        }
        println!("{:?}", laser_count_row);
        let mut result: i32 = 0;
        if laser_count_row.len() <= 1{return result}
        for i in 0..laser_count_row.len()-1{    
            result += (laser_count_row[i] *  laser_count_row[i+1]);
        }

        result
    }
}