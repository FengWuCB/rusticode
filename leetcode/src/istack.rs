mod iqueue;

struct MyStack {
    q: crate::iqueue::MyQueue,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    /** Initialize your data structure here. */
    fn new() -> Self {
        MyStack{
            q: crate::iqueue::MyQueue::new(),
        }  
    }
    
    /** Push element x onto stack. */
    // 思路：逆序入队，顺序出队
    fn push(&mut self, x: i32) {
        let mut cache = crate::iqueue::MyQueue::new();
        while !self.empty() {
            cache.push_back(self.pop());
        }
        self.q.push_back(x);
        while cache.len() > 0 {
            self.q.push_back(cache.pop());
        }
    }
    
    /** Removes the element on top of the stack and returns that element. */
    fn pop(&mut self) -> i32 {
        return self.q.pop();
    }
    
    /** Get the top element. */
    fn top(&self) -> i32 {
        return self.q.top();
    }
    
    /** Returns whether the stack is empty. */
    fn empty(&self) -> bool {
        return self.q.is_empty();
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

fn main()
{
    let mut istack = MyStack::new();
    istack.push(42);
    println!("top element: {}", istack.top());
    println!("empty stack?: {}", istack.empty());
    println!("pop element: {}", istack.pop());
    println!("empty stack?: {}", istack.empty());

    println!("push in: ");
    for i in (0..8).step_by(2) {
        print!("{:3}  ", i);
        istack.push(i);
    }
    println!("");

    println!("pop out: ");
    for _ in (0..8).step_by(2) {
        print!("{:3}  ", istack.pop());
    }
    println!("");

}