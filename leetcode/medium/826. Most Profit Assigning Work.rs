impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut diff_prof: Vec<Vec<i32>> = vec![];
        let mut temp_w: Vec<i32> = worker.to_vec();
        let mut result: i32 = 0;

        for i in 0..difficulty.len() {
            diff_prof.push(vec![difficulty[i], profit[i]]);
        }
        temp_w.sort_by(|a, b| a.cmp(b));
        diff_prof.sort_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1])
            }
        });
        // println!("difficulty, profit : {:?}", diff_prof);
        // println!("worker: {:?}", temp_w);

        let mut curr_profit: i32 = 0;
        let mut curr_i: usize = 0;
        for w_i in 0..temp_w.len() {
            let mut idx: usize = 0;
            while idx < diff_prof.len() && temp_w[w_i] >= diff_prof[idx][0] {
                if curr_profit < diff_prof[idx][1] {
                    curr_profit = diff_prof[idx][1];
                }
                idx += 1;
            }
            // if curr_profit == i32::MIN {break}
            // println!("profit: {}", curr_profit);
            result += curr_profit;
        }

        result
    }
}
