impl Solution {
    pub fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut seats_sorted: Vec<i32> = seats.clone();
        let mut studs_sorted: Vec<i32> = students.clone();
        seats_sorted.sort_by(|a, b| a.cmp(b));
        studs_sorted.sort_by(|a, b| a.cmp(b));
        let mut count: i32 = 0;
        // println!("seats:{:?}\nstuds:{:?}", seats_sorted, studs_sorted);
        for i in 0..studs_sorted.len() {
            count += i32::abs(studs_sorted[i] - seats_sorted[i]);
        }

        count
    }
}
