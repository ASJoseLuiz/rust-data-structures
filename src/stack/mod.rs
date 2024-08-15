pub mod stack;

#[cfg(test)]
mod tests {
    use super::stack::{Stack, StackTrait, MAX};

    #[test]
    fn test_push_pop() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());

        stack.push(10).unwrap();
        stack.push(20).unwrap();
        stack.push(30).unwrap();

        assert_eq!(stack.pop(), Some(30));
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));
        assert!(stack.is_empty());
    }

    #[test]
    fn test_is_full() {
        let mut stack = Stack::new();
        for i in 0..MAX {
            stack.push(i).unwrap();
        }
        assert!(stack.is_full());
    }

    #[test]
    fn test_overflow() {
        let mut stack = Stack::new();
        for i in 0..MAX {
            stack.push(i).unwrap();
        }
        assert!(stack.push(101).is_err());
    }

    #[test]
    fn test_underflow() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.pop().is_none());
    }
}
