use std::collections::VecDeque;
impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let mut queue: VecDeque<i32> = VecDeque::new();
        // let mut queue_2: VecDeque<i32> = VecDeque::new();
        let mut time: i32 = 0;
        let mut curr_pos: i32 = k;
        for i in 0..tickets.len() {
            queue.push_back(tickets[i]);
        }
        // println!("{:?}", queue);
        let mut idx: usize = 0;
        while queue.len() > 0 && *queue.get(curr_pos as usize).unwrap() > 0 {
            // println!("{:?} : {}", queue, curr_pos);
            let mut front: i32 = queue.pop_front().unwrap();
            front -= 1;
            time += 1;
            // checking if the choosen person already bought all the tickets that he/she needed
            let temp = front;
            queue.push_front(front);
            let mut the_choosen: i32 = *queue.get(curr_pos as usize).unwrap();
            if the_choosen <= 0 {
                break;
            }
            let mut front: i32 = queue.pop_front().unwrap();
            //
            curr_pos -= 1;
            if front > 0 {
                queue.push_back(front);
            }
            if curr_pos < 0 {
                curr_pos = (queue.len() - 1) as i32;
            }
            // println!("{:?} : {}\ntime:{}", queue, curr_pos, time);
            // println!("-----");
            // break;
        }

        time
    }
}
