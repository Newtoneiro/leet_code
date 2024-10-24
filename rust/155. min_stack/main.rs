use std::collections::VecDeque;

struct MinStack {
    internal_stack: VecDeque<(i32, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            internal_stack: VecDeque::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        if let Some((_, min)) = self.internal_stack.back() {
            self.internal_stack.push_back((val, *min.min(&val)));
        } else {
            self.internal_stack.push_back((val, val));
        }
    }
    
    fn pop(&mut self) {
        self.internal_stack.pop_back();
    }
    
    fn top(&self) -> i32 {
        self.internal_stack.back().unwrap().0
    }
    
    fn get_min(&self) -> i32 {
        self.internal_stack.back().unwrap().1
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */