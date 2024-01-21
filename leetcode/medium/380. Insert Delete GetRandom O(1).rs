use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;

struct RandomizedSet {
    // num_map: HashMap<i32, i32>,
    nums_set: HashSet<i32>,
    nums: Vec<i32>,
}

///**
// * `&self` means the method takes an immutable reference.
// * If you need a mutable reference, change it to `&mut self` instead.
// */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            // num_map: HashMap::new(),
            nums_set: HashSet::new(),
            nums: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        let mut is_exist: bool = self.nums_set.contains(&val);
        self.nums.push(val);
        self.nums_set.insert(val);
        // if let Some(freq) = self.num_map.get_mut(&val) {
        //     *freq += 1;
        // } else {
        //     self.num_map.insert(val, 1);
        // }

        !is_exist
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.nums_set.contains(&val) {
            return false;
        }
        // if let Some(freq) = self.num_map.get_mut(&val) {
        //     *freq -= 1;
        //     if *freq == 0 {
        //         self.num_map.remove(&val);
        //     }
        // }
        self.nums_set.remove(&val);
        let mut idx: usize = 0;
        while idx < self.nums.len() {
            if self.nums[idx] == val {
                break;
            }
            idx += 1;
        }
        self.nums.remove(idx);
        true
    }

    fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let length = self.nums.len() as f64;
        let mut idx: f64 = rng.gen();
        idx *= length;
        // println!("{}", idx as usize);

        self.nums[idx as usize]
    }
}

// /**
// * Your RandomizedSet object will be instantiated and called as such:
// * let obj = RandomizedSet::new();
// * let ret_1: bool = obj.insert(val);
// * let ret_2: bool = obj.remove(val);
// * let ret_3: i32 = obj.get_random();
// */
