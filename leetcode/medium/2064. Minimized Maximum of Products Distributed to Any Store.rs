impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut start: i32 = 1;
        let mut end: i32 = *quantities.iter().max().unwrap();
        while start < end{
            let mid: i32 = (start+end)/2;
            if Self::is_enough(mid, &quantities, n){
                end = mid;
                continue;
            }
            start = mid + 1;
        }
        return start;
    }

    pub fn is_enough(quantity: i32, products: &Vec<i32>, stores: i32)->bool{
        let mut count_stores: i32 = 0;
        for product in products{
            let mut temp_quan: i32 = *product;
            count_stores += if temp_quan % quantity == 0 {temp_quan/quantity} else {(temp_quan/quantity)+1};
            if count_stores > stores{
                return false;
            }
        }
        return true;
    }
}
