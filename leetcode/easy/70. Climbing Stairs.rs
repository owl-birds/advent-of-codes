impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut prev: i32 = 0;
        let mut curr: i32 = 1;
        for i in 0..n {
            let temp = prev;
            prev = curr;
            curr += temp;
            println!("{}", curr);
        }
        curr
    }
}
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }

        let mut before_last: i32 = 1;
        let mut last: i32 = 2;

        for _ in 3..=n {
            let next = before_last + last;
            before_last = last;
            last = next;
        }

        last
    }
}
