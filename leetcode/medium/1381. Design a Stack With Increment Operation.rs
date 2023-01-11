struct CustomStack {
    storage: Vec<i32>,
    max_size: i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        CustomStack{
            storage: vec![], max_size: maxSize
        }
    }
    
    fn push(&mut self, x: i32) {
        if self.storage.len() < self.max_size as usize{
            self.storage.push(x);
        }
    }
    
    fn pop(&mut self) -> i32 {
        if self.storage.len() == 0{
            return -1;
        }
        let top_num: i32 = self.storage[self.storage.len()-1];
        self.storage.pop();
        return top_num;
    }
    
    fn increment(&mut self, k: i32, val: i32) {
        for i in 0..self.storage.len(){
            if i >= k as usize{
                break;
            }
            self.storage[i] += val;
        }
    }
}

/**
 * Your CustomStack object will be instantiated and called as such:
 * let obj = CustomStack::new(maxSize);
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * obj.increment(k, val);
 */