impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        // println!("{:?}",Self::dec_to_bin(4));
        // println!("{:?}",Self::dec_to_bin(1));
        // println!("{:?}",Self::dec_to_bin(156));
        let mut count: i32 = 0;

        let mut longer_bin: Vec<i32> = Self::dec_to_bin(if x > y{x}else{y});
        let mut shorter_bin: Vec<i32> = Self::dec_to_bin(if x > y{y}else{x});
        println!("{:?}", longer_bin);
        println!("{:?}", shorter_bin);

        for i in 0..shorter_bin.len(){
            if shorter_bin[i] != longer_bin[i]{
                count += 1;
            }
        }

        for i in shorter_bin.len()..longer_bin.len(){
            if 0 != longer_bin[i]{
                count += 1;
            }
        }

        count
    }
    pub fn dec_to_bin(dec_num: i32)->Vec<i32>{
        let mut bin: Vec<i32> = vec![];
        let mut temp_num: i32 = dec_num;

        while temp_num > 0{
            if temp_num % 2 == 0{
                // bin.insert(0, 0);
                bin.push(0);
            }else{
                // bin.insert(0, 1);
                bin.push(1);
            }
            temp_num = temp_num/2;
        }

        bin
    }
}
