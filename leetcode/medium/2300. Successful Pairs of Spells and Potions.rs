impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut successful = vec![];

        // ape smooth brain way
        // TLE
        // for i in 0..spells.len(){
        //     let mut count_success = 0;
        //     for j in 0..potions.len(){
        //         if spells[i] as i64 * potions[j] as i64 >= success  {count_success += 1}
        //     }
        //     successful.push(count_success);
        // }

        // sorting options first
        let mut sorted_potions = potions.clone();
        sorted_potions.sort();
        // println!("{:?}", sorted_potions);

        // STILL TLE
        // for i in 0..spells.len(){
        //     let mut count_success = 0;
        //     for j in 0..sorted_potions.len(){
        //         if spells[i] as i64 * sorted_potions[j] as i64 >= success{
        //             count_success += sorted_potions.len() as i32 - j as i32;
        //             break;
        //         }
        //     }
        //     successful.push(count_success);
        // }

        // add some binary search
        for i in 0..spells.len(){
            let mut start = 0;
            let mut end = sorted_potions.len()-1;
            while start < end{
                let mid = (start+end)/2;
                if (spells[i] as i64 * sorted_potions[mid] as i64) < success{
                    start = mid+1;
                    continue;
                }
                end = mid;
            }
            if start == sorted_potions.len()-1 && (spells[i] as i64 * sorted_potions[start] as i64) >= success{ 
                successful.push(1);
                continue;
            }
            if start == sorted_potions.len()-1 && (spells[i] as i64 * sorted_potions[start] as i64) < success{
                successful.push(0);
                continue;
            }
            successful.push((sorted_potions.len()-start) as i32);
        }

        successful
    }
}