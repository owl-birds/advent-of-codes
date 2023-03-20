impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0{return true;}
        let mut temp_n = n;
        let mut temp_bed = flowerbed.clone();
        let bed_length = temp_bed.len();
        if temp_bed[0] == 0 && temp_bed.len() == 1{
            return true;
        }
        if temp_bed[0] == 1 && temp_bed.len() == 1 && temp_n > 0{
            return false;
        }
        if temp_bed[temp_bed.len()-1] == 0 && temp_bed[temp_bed.len()-2] == 0{
            temp_bed[bed_length-1] = 1;
            temp_n -= 1;
        }    

    
        for i in 0..temp_bed.len(){
            if temp_n == 0 {return true;}
            if i == bed_length-1 && temp_bed[i-1] == 0{
                temp_bed[i] = 1;
                temp_n -= 1;
                continue;
            }
            if i == 0 && temp_bed[i] == 0 && temp_bed[i+1] == 0{
                temp_bed[i] = 1;
                temp_n -= 1;
                continue;
            }
            if i == 0{continue;}
            if temp_bed[i-1] == 0 && temp_bed[i] == 0 && temp_bed[i+1] == 0{
                temp_bed[i] = 1;
                temp_n -= 1;
                continue;
            }
        }

        return false;
    }
}

