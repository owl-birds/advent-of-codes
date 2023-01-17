impl Solution {

    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {

        

        // fn is_enough(capacity: i32, days: i32, weights: &Vec<i32>) -> bool{

        //     let mut total: i32 = 0;

        //     let mut count_days: i32 = 1; // starting day

        //     // for i in 0..weights.len(){

        //     for weight in weights.iter(){

        //         total += *weight;

        //         if total > capacity{

        //             total = *weight;

        //             count_days += 1;

        //             if count_days > days{

        //                 return false;

        //             }

        //         }

        //     }

        //     return true;

        // }

        let mut start_capacity: i32 = *weights.iter().max().unwrap();;

        let mut end_capacity: i32 = weights.iter().sum::<i32>();;



        while start_capacity < end_capacity{

            let mid_capacity: i32 = start_capacity + (end_capacity-start_capacity)/2;

            if Self::is_enough(mid_capacity, days, &weights){

                end_capacity = mid_capacity;

                continue;

            }

            start_capacity = mid_capacity + 1;

        }



        return start_capacity;

    }

    pub fn is_enough(capacity: i32, days: i32, weights: &Vec<i32>) -> bool{

            let mut total: i32 = 0;

            let mut count_days: i32 = 1; // starting day

            // for i in 0..weights.len(){

            for weight in weights.iter(){

                total += *weight;

                if total > capacity{

                    total = *weight;

                    count_days += 1;

                    if count_days > days{

                        return false;

                    }

                }

            }

            return true;

        }

}