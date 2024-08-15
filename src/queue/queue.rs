pub const MAX: usize = 100;

pub trait QueueTrait<T> {
    fn new() -> Self;
    fn enqueue(&mut self, element: T) -> Result<(), String>;
    fn dequeue(&mut self) -> Result<T, String>;
    fn head(&self) -> Option<&T>;
    fn tail(&self) -> Option<&T>;
    fn is_empty(&self) -> bool;
    fn is_full(&self) -> bool;
}

pub struct Queue<T> {
    array: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T> QueueTrait<T> for Queue<T> {
    fn new() -> Self {
        Queue {
            array: Vec::with_capacity(MAX),
            head: 0,
            tail: 0,
            size: 0,
        }
    }
    fn enqueue(&mut self, element: T) -> Result<(), String> {
        if self.is_full() {
            return Err("A fila está cheia".to_string());
        }
        if self.tail == self.array.len() {
            self.array.push(Some(element));
        } else {
            self.array[self.tail] = Some(element);
        }
        self.tail = (self.tail + 1) % MAX;
        self.size += 1;
        Ok(())
    }
    
    fn dequeue(&mut self) -> Result<T, String> {
        if self.is_empty() {
            return Err("A fila está vazia".to_string());
        }
        let element = self.array[self.head].take();
        self.head = (self.head + 1) % MAX;
        self.size -= 1;
        element.ok_or("Erro ao desempilhar".to_string())
    }
    
    fn head(&self) -> Option<&T> {
        self.array[self.head].as_ref()
    }
    
    fn tail(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.array[(self.tail + MAX - 1) % MAX].as_ref()
        }
    }
    
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    fn is_full(&self) -> bool {
        self.size == MAX
    }
}


