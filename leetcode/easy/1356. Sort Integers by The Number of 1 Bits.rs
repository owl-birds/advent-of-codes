impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut temp: Vec<Vec<i32>> = vec![];

        for num in arr {
            temp.push(vec![num, Self::count_one(num)]);
        }
        temp.sort_by(|a, b| {
            if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                a[0].cmp(&b[0])
            }
        });
        let mut result: Vec<i32> = vec![];
        for num in temp {
            result.push(num[0]);
        }

        result
    }
    pub fn count_one(num: i32) -> i32 {
        let mut count: i32 = 0;
        let mut temp: i32 = num;
        while temp > 0 {
            if temp % 2 != 0 {
                count += 1
            }
            temp /= 2;
        }
        count
    }
}
