use std::collections::VecDeque;
struct MyStack {
    store: VecDeque<i32>,
}

// /**
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
impl MyStack {
    fn new() -> Self {
        Self {
            store: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.store.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        if let Some(top_ele) = self.store.pop_back() {
            return top_ele;
        }
        -1
    }

    fn top(&self) -> i32 {
        if let Some(top_ele) = self.store.back() {
            return *top_ele;
        }
        -1
    }

    fn empty(&self) -> bool {
        self.store.len() == 0
    }
}

// /**
//  * Your MyStack object will be instantiated and called as such:
//  * let obj = MyStack::new();
//  * obj.push(x);
//  * let ret_2: i32 = obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: bool = obj.empty();
//  */
