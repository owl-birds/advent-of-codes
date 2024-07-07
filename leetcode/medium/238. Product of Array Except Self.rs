impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut pref: Vec<i32> = vec![];
        let mut suff: Vec<i32> = vec![];

        let mut idx: usize = nums.len();
        let mut p: i32 = 1;
        let mut s: i32 = 1;

        for i in 0..nums.len() {
            idx -= 1;
            p *= nums[i];
            s *= nums[idx];
            pref.push(p);
            suff.push(s);
        }
        suff.reverse();
        // println!("{:?}\n{:?}", pref, suff);
        for i in 0..nums.len() {
            if i == 0 {
                res.push(suff[1]);
                continue;
            }
            if i == nums.len() - 1 {
                res.push(pref[i - 1]);
                continue;
            }
            res.push(pref[i - 1] * suff[i + 1]);
        }

        res
    }
}
