/**
 * Your SeatManager object will be instantiated and called as such:
 * let obj = SeatManager::new(n);
 * let ret_1: i32 = obj.reserve();
 * obj.unreserve(seatNumber);
 */
// use std::collections::HashMap;
use std::collections::HashSet;
struct SeatManager {
    // seats: Vec<bool>,
    // pointer: usize,
    total_seats: i32,
    unreserved: Vec<i32>,
    // reserved: Vec<i32>
    reserved: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SeatManager {
    fn new(n: i32) -> Self {
        // let mut new_seats: Vec<bool> = vec![];
        // for i in 0..n {
        //     new_seats.push(false);
        // }
        let mut new_unreserved: Vec<i32> = vec![];
        let mut seat: i32 = n;
        while seat > 0 {
            new_unreserved.push(seat);
            seat -= 1;
        }
        Self {
            // seats: new_seats,
            // pointer: 0
            total_seats: n,
            unreserved: new_unreserved,
            // reserved: vec![]
            reserved: HashSet::new(),
        }
    }

    fn reserve(&mut self) -> i32 {
        // println!("{:?}\n{:?}", self.unreserved, self.reserved);
        if self.unreserved.len() == 0 {
            return -1;
        }
        let mut seat = self.unreserved.pop().unwrap();
        self.reserved.insert(seat);

        seat

        // if self.pointer < self.seats.len() {
        //     self.seats[self.pointer] = true;
        //     let result: i32 = self.pointer as i32 + 1;
        //     // self.pointer += 1;
        //     for i in 0..self.seats.len() {
        //         if !self.seats[i] {
        //             self.pointer = i;
        //             break;
        //         }
        //         if i == (self.seats.len()-1) {
        //             self.pointer = self.seats.len();
        //         }
        //     }
        //     return result;
        // }

        // -1

        // let mut idx: usize = 0;
        // while idx < self.seats.len() {
        //     if !self.seats[idx] {break}
        //     idx += 1;
        // }
        // if idx == self.seats.len() {return -1}
        // self.seats[idx] = true;

        // (idx+1) as i32
    }

    fn unreserve(&mut self, seat_number: i32) {
        if !self.reserved.contains(&seat_number) || seat_number > self.total_seats {
            return;
        }
        self.reserved.remove(&seat_number);
        if self.unreserved.len() == 0 || seat_number < self.unreserved[self.unreserved.len() - 1] {
            self.unreserved.push(seat_number);
        } else {
            let mut idx: usize = self.unreserved.len();
            while idx >= 1 {
                idx -= 1;
                if self.unreserved[idx] > seat_number {
                    break;
                }
            }
            self.unreserved.insert(idx + 1, seat_number);
        }

        // self.seats[seat_number as usize - 1] = false;
        // for i in 0..seat_number as usize {
        //     if !self.seats[i] {
        //         self.pointer = i;
        //         break;
        //     }
        // }
    }
}
