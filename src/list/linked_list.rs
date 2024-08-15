pub trait LinkedListTrait<T> {
    fn is_empty(&self) -> bool;
    fn size(&self) -> usize;
    fn insert(&mut self, element: T);
    fn delete(&mut self, element: &T) -> Result<(), String>;
    fn search(&self, element: &T) -> Result<&T, String>;
    fn to_array(&self) -> Vec<T>;
}

pub struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T>{
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Clone + PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {head: None, size: 0}
    }
}

impl<T: Clone + PartialEq> LinkedListTrait<T> for LinkedList<T> {
    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn insert(&mut self, element: T) { // Inserção no Fim da Lista
        let new_node = Box::new(Node {
            data: element,
            next: None,
        });

        match self.head {
            Some(ref mut head) => {
                let mut aux_node = head.as_mut();
                while let Some(ref mut next_node) = aux_node.next {
                    aux_node = next_node.as_mut();
                }
                aux_node.next = Some(new_node);
            }

            None => {
                self.head = Some(new_node);
            }
        }

        self.size += 1;

    }

    fn delete(&mut self, element: &T) -> Result<(), String> {
        {
            let head = &mut self.head;
            if let Some(ref mut node) = head {
                if &node.data == element {
                    *head = node.next.take();
                    self.size -= 1;
                    return Ok(());
                }
            }
        }

        let mut aux_node = &mut self.head;
        while let Some(ref mut node) = aux_node {
            if let Some(ref mut next_node) = node.next {
                if &next_node.data == element {
                    node.next = next_node.next.take();
                    self.size -= 1;
                    return Ok(());
                }
            }
            aux_node = &mut node.next;
        }

        Err("Elemento não encontrado".to_string())
    }
    

    fn search(&self, element: &T) -> Result<&T, String> {
        let mut aux_node = &self.head;
        while let Some(ref node) = aux_node {
            if &node.data == element {
                return Ok(&node.data);
            }
            aux_node = &node.next;
        }
        Err("Elemento não encontrado".to_string())
    }

    fn to_array(&self) -> Vec<T> {
        let mut array = Vec::with_capacity(self.size);
        let mut aux_node = &self.head;

        while let Some(ref node) = aux_node {
            array.push(node.data.clone());
            aux_node = &node.next;
        }

        array
    }
}