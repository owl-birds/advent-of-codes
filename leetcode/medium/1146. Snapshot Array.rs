use std::collections::HashMap;
struct SnapshotArray {
    // Memory Limit Exceededv
    // array: Vec<i32>,
    // snapshot_map: HashMap<i32, Vec<i32>>,
    // Memory Limit Exceededv
    array_snapshot: Vec<HashMap<i32, i32>>,
    count_snap: i32,
}

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        // Memory Limit Exceededv
        // let mut new_arr: Vec<i32> = vec![];
        // Memory Limit Exceededv

        let mut new_snap_arr: Vec<HashMap<i32, i32>> = vec![];
        let curr_snap: i32 = 0;
        for i in 0..length {
            // Memory Limit Exceededv
            // new_arr.push(0);
            // Memory Limit Exceededv
            new_snap_arr.push(HashMap::from([(curr_snap, 0)]));
        }
        // println!("{:?}", new_snap_arr);
        Self {
            // Memory Limit Exceededv
            // array: new_arr,
            // snapshot_map: HashMap::new(),
            // Memory Limit Exceededv
            array_snapshot: new_snap_arr,
            count_snap: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        // Memory Limit Exceededv
        // self.array[index as usize] = val;
        // Memory Limit Exceededv
        self.array_snapshot[index as usize].insert(self.count_snap, val);
        // println!("set:{:?}", self.array_snapshot);
    }

    fn snap(&mut self) -> i32 {
        // Memory Limit Exceededv
        // self.snapshot_map.insert(self.count_snap, self.array.clone());
        // Memory Limit Exceededv

        // SLOW
        // everytime we snap, we add  to all index new snap_id, and its prev value
        // if self.count_snap > 0{
        //     for i in 0..self.array_snapshot.len(){
        //         // get the prev value from the prev snap_id
        //         if self.array_snapshot[i].contains_key(&self.count_snap) {continue;}
        //         let mut prev_val: i32 = 0;
        //         let get_snap = self.array_snapshot[i].get(&(self.count_snap-1));
        //         match get_snap {
        //             Some(prev) => prev_val = *prev,
        //             None => prev_val = 0
        //         }
        //         self.array_snapshot[i].insert(self.count_snap, prev_val);
        //     }
        // }
        // println!("snap:{:?}", self.array_snapshot);
        // SLOW

        self.count_snap += 1;
        self.count_snap - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let get_snap = self.array_snapshot[index as usize].get(&snap_id);
        match get_snap {
            Some(val) => return *val,
            None => {
                // BUG BELOW
                let mut temp_keys: Vec<i32> = vec![];
                for key in self.array_snapshot[index as usize].keys() {
                    temp_keys.push(*key);
                }
                // println!("{:}")
                // let max_key: i32 = *temp_keys.iter().max().unwrap();
                temp_keys.sort();
                let mut the_snap_key: i32 = temp_keys[0];
                for i in 0..temp_keys.len() {
                    if temp_keys[i] > snap_id {
                        break;
                    }
                    the_snap_key = temp_keys[i];
                }

                // dont find max key, but find the gap for ex : [0,2,4] : snap_id 1 so we find value 0, or if we find snap_id 3: find num berfore that snap_id
                let get_prev_snap_in_curr_index =
                    self.array_snapshot[index as usize].get(&the_snap_key);
                match get_prev_snap_in_curr_index {
                    Some(prev_val) => return *prev_val,
                    None => return -1,
                }
            }
        }

        // Memory Limit Exceededv
        // let get_snap = self.snapshot_map.get(&snap_id);
        // match get_snap {
        //     Some(snap) => return snap[index as usize],
        //     None => return -1
        // }
        // Memory Limit Exceededv
    }
}

// /**
//  * Your SnapshotArray object will be instantiated and called as such:
//  * let obj = SnapshotArray::new(length);
//  * obj.set(index, val);
//  * let ret_2: i32 = obj.snap();
//  * let ret_3: i32 = obj.get(index, snap_id);
//  */
