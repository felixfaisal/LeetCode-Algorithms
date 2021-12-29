struct MinStack {
    stack1: Vec<i32>,
    stack2: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        MinStack{
            stack1: vec![], 
            stack2: vec![],
        }
    }
    
    fn push(&mut self, val: i32) {
        self.stack1.push(val); 
        if self.stack2.len() == 0 {
            self.stack2.push(val); 
        }
        else{
            let top = self.stack2[self.stack2.len()-1];
            if top > val {
                self.stack2.push(val);
            }
            else {
                self.stack2.push(top);
            }
        }
    }
    
    fn pop(&mut self) {
        self.stack1.pop();
        self.stack2.pop();
    }
    
    fn top(&self) -> i32 {
        self.stack1[self.stack1.len()-1]
    }
    
    fn get_min(&self) -> i32 {
        self.stack2[self.stack2.len()-1]
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