impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut new: Vec<Vec<i32>> = vec![];
        if intervals.len() == 0 {
            new.push(new_interval.to_vec());
            return new;
        }
        let mut included: Vec<usize> = vec![];

        for i in 0..intervals.len() {
            if intervals[i][0] <= new_interval[0] && intervals[i][1] >= new_interval[0] {
                included.push(i);
                continue;
            }
            if intervals[i][0] <= new_interval[1] && intervals[i][1] >= new_interval[1] {
                included.push(i);
                continue;
            }
            if new_interval[0] <= intervals[i][0] && new_interval[1] >= intervals[i][0] {
                included.push(i);
                continue;
            }
            if new_interval[0] <= intervals[i][1] && new_interval[1] >= intervals[i][1] {
                included.push(i);
                continue;
            }
            new.push(intervals[i].to_vec());
        }
        // println!("{:?}", included);
        if included.len() > 0 {
            let mut new_vec: Vec<i32> = vec![];
            new_vec.push(if intervals[included[0]][0] > new_interval[0] {
                new_interval[0]
            } else {
                intervals[included[0]][0]
            });
            new_vec.push(
                if intervals[included[included.len() - 1]][1] < new_interval[1] {
                    new_interval[1]
                } else {
                    intervals[included[included.len() - 1]][1]
                },
            );
            // println!("{:?}", new);
            new.insert(included[0], new_vec);
        } else if new_interval[0] < intervals[0][0]
            && new_interval[0] < intervals[0][1]
            && new_interval[1] < intervals[0][0]
            && new_interval[1] < intervals[0][1]
        {
            new.insert(0, new_interval.to_vec());
        } else if new_interval[0] > intervals[intervals.len() - 1][0]
            && new_interval[0] > intervals[intervals.len() - 1][1]
            && new_interval[1] > intervals[intervals.len() - 1][0]
            && new_interval[1] > intervals[intervals.len() - 1][1]
        {
            new.push(new_interval.to_vec());
        } else {
            for i in 0..intervals.len() {
                if new_interval[0] < intervals[i][0]
                    && new_interval[0] < intervals[i][1]
                    && new_interval[1] < intervals[i][0]
                    && new_interval[1] < intervals[i][1]
                {
                    new.insert(i, new_interval.to_vec());
                    break;
                }
            }
        }
        new
    }
}
