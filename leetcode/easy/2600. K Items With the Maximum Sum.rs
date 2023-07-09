impl Solution {
    pub fn k_items_with_maximum_sum(
        num_ones: i32,
        num_zeros: i32,
        num_neg_ones: i32,
        k: i32,
    ) -> i32 {
        let mut temp_k: i32 = k;
        let ones: i32 = if temp_k >= num_ones {
            temp_k -= num_ones;
            num_ones
        } else {
            let temp_k_2 = temp_k;
            temp_k = 0;
            temp_k_2
        };
        let zeros: i32 = if temp_k >= num_zeros {
            temp_k -= num_zeros;
            0
        } else {
            temp_k = 0;
            0
        };
        let neg_ones: i32 = if temp_k >= num_neg_ones {
            temp_k -= num_neg_ones;
            num_neg_ones
        } else {
            let temp_k_2 = temp_k;
            temp_k = 0;
            temp_k_2
        };
        // println!("ones:{}\nzeros:{}\nneg_ones:{}", ones, zeros, neg_ones);

        ones + zeros - neg_ones
    }
}
