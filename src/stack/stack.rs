
pub const MAX: usize = 100;
pub trait StackTrait<T> {
    fn push(&mut self, element: T) -> Result<(), String>; 
    fn pop(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

pub struct Stack<T> {
    array: Vec<T>,
    top: usize,
}

impl<T: PartialOrd> Stack<T> {
    pub fn new() -> Self {
        Stack {
            array: Vec::with_capacity(MAX),
            top: 0,
        }
    }

    pub fn get_top(&self) -> usize {
        self.top
    }

    pub fn set_top(&mut self, index: usize ) {
        self.top = index;
    }
}

impl<T: PartialOrd> StackTrait<T> for Stack<T> {
    
    fn push(&mut self, element: T) -> Result<(), String>{
        if self.is_full() {
            return Err("A pilha está cheia".to_string());
        }
        self.array.push(element);
        self.set_top(self.get_top() + 1);
        Ok(())
   }

    fn pop(&mut self) -> Option<T>{
        if self.is_empty() {
            return None;
        }
        let element = self.array.pop();
        self.set_top(self.get_top() - 1);
        element
    }

    fn is_empty(&self) -> bool {
        self.top == 0
    }

    fn is_full(&self) -> bool {
        self.top == MAX
    }
}