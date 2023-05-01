impl Solution {
    pub fn average(salary: Vec<i32>) -> f64 {
        let mut max = salary[0];
        let mut min = salary[0];
        let mut sum = 0;
        for i in 0..salary.len(){
            if max < salary[i]{max = salary[i]}
            if min > salary[i]{min = salary[i]}
            sum += salary[i];
        } 
        (sum - max - min) as f64 / (salary.len() as f64 - 2.0)
    }
}