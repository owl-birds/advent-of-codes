use std::collections::VecDeque;
impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut stud_q: VecDeque<i32> = VecDeque::new();
        let mut sand_q: VecDeque<i32> = VecDeque::new();
        for i in 0..students.len() {
            stud_q.push_back(students[i]);
            sand_q.push_back(sandwiches[i]);
        }
        // println!("students:{:?}\nsandwiches:{:?}", stud_q, sand_q);
        let mut count_cycle: i32 = 0; // it will be refreshed if a student get to eat the sandwich of his/her choice

        while stud_q.len() > 0 && sand_q.len() > 0 && count_cycle < stud_q.len() as i32 {
            let mut front_stud: i32 = *stud_q.front().unwrap();
            let mut front_sand: i32 = *sand_q.front().unwrap();
            if front_stud == front_sand {
                stud_q.pop_front();
                sand_q.pop_front();
                count_cycle = 0;
            } else {
                let mut popped_stud: i32 = stud_q.pop_front().unwrap();
                stud_q.push_back(popped_stud);
                count_cycle += 1;
            }
            // break
        }
        // println!("students:{:?}\nsandwiches:{:?}", stud_q, sand_q);

        stud_q.len() as i32
    }
}
