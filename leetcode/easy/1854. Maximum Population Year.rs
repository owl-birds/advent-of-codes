impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        // let mut sorted_logs: Vec<Vec<i32>> = vec![];
        // for log in logs {
        //     sorted_logs.push(log.to_vec());
        // }
        // sorted_logs.sort_by(|a, b| {
        //     if a[0] != b[0] {a[0].cmp(&b[0])}
        //     else {b[1].cmp(&a[1])}
        // });
        // println!("logs:{:?}", sorted_logs);
        // let mut earliest_year: [i32; 2] = [i32::MAX, i32::MAX];
        let mut earliest_year: i32 = i32::MAX;
        let mut max_pop: i32 = 0;

        for i in 0..logs.len() {
            let mut count_pop: i32 = 1;
            // if earliest_year[0] == logs[i][0] || earliest_year[1] == logs[i][1] {continue;}
            for j in 0..logs.len() {
                if i == j {
                    continue;
                }
                if logs[j][0] <= logs[i][0] && logs[j][1] > logs[i][0] {
                    count_pop += 1;
                }
            }
            // println!("{:?}--->{}", logs[i], count_pop);
            if count_pop > max_pop {
                max_pop = count_pop;
                earliest_year = logs[i][0];
                // earliest_year[0] = logs[i][0];
                // earliest_year[1] = logs[i][1];
            }
            if count_pop == max_pop && earliest_year > logs[i][0] {
                earliest_year = logs[i][0];
            }
        }

        // earliest_year[0]
        earliest_year
    }
}
