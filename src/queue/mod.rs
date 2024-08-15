pub mod queue;

#[cfg(test)]
mod tests {
    use crate::queue::queue::{Queue, QueueTrait, MAX};

    #[test]
    fn test_enqueue_dequeue() {
        let mut queue = Queue::new();

        // Testa enqueue
        assert!(queue.enqueue(1).is_ok());
        assert!(queue.enqueue(2).is_ok());
        assert!(queue.enqueue(3).is_ok());

        // Testa head e tail
        assert_eq!(queue.head(), Some(&1));
        assert_eq!(queue.tail(), Some(&3));

        // Testa dequeue
        assert_eq!(queue.dequeue(), Ok(1));
        assert_eq!(queue.dequeue(), Ok(2));
        assert_eq!(queue.dequeue(), Ok(3));

        // Testa dequeue em fila vazia
        assert!(queue.dequeue().is_err());
    }

    #[test]
    fn test_is_empty() {
        let mut queue = Queue::new();
        assert!(queue.is_empty());

        queue.enqueue(1).unwrap();
        assert!(!queue.is_empty());

        queue.dequeue().unwrap();
        assert!(queue.is_empty());
    }

    #[test]
    fn test_is_full() {
        let mut queue = Queue::new();

        // Enche a fila
        for i in 0..MAX {
            queue.enqueue(i).unwrap();
        }
        assert!(queue.is_full());

        // Remove um elemento e testa
        queue.dequeue().unwrap();
        assert!(!queue.is_full());
    }

    #[test]
    fn test_circular_behavior() {
        let mut queue = Queue::new();

        // Enfileira elementos até encher a fila
        for i in 0..MAX {
            queue.enqueue(i).unwrap();
        }

        // Remove alguns elementos
        for _ in 0..10 {
            queue.dequeue().unwrap();
        }

        // Adiciona mais elementos para verificar comportamento circular
        for i in 100..110 {
            queue.enqueue(i).unwrap();
        }

        // Verifica os elementos na fila
        assert_eq!(queue.head(), Some(&10));
        assert_eq!(queue.tail(), Some(&109));
    }

    #[test]
    fn test_enqueue_full_queue() {
        let mut queue = Queue::new();

        // Enche a fila
        for i in 0..MAX {
            queue.enqueue(i).unwrap();
        }

        // Tenta enfileirar mais um elemento
        assert!(queue.enqueue(100).is_err());
    }
}
