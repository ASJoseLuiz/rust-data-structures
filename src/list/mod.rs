pub mod linked_list;

#[cfg(test)]
mod tests {
    use crate::list::linked_list::{LinkedList, LinkedListTrait};

    #[test]
    fn test_insert() {
        let mut list = LinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        assert_eq!(list.size(), 3);

        let array = list.to_array();
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn test_delete() {
        let mut list = LinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);

        assert_eq!(list.delete(&2), Ok(()));
        assert_eq!(list.size(), 2);

        let array = list.to_array();
        assert_eq!(array, vec![1, 3]);

        assert_eq!(list.delete(&1), Ok(()));
        assert_eq!(list.size(), 1);

        let array = list.to_array();
        assert_eq!(array, vec![3]);

        assert_eq!(list.delete(&3), Ok(()));
        assert_eq!(list.size(), 0);

        let array = list.to_array();
        assert!(array.is_empty());

        assert_eq!(list.delete(&4), Err("Elemento não encontrado".to_string()));
    }

    #[test]
    fn test_search() {
        let mut list = LinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);

        assert_eq!(list.search(&2), Ok(&2));
        assert_eq!(list.search(&3), Ok(&3));

        assert_eq!(list.search(&4), Err("Elemento não encontrado".to_string()));
    }

    #[test]
    fn test_to_array() {
        let mut list = LinkedList::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);

        let array = list.to_array();
        assert_eq!(array, vec![1, 2, 3]);
    }

    #[test]
    fn test_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());

        let mut list = LinkedList::new();
        list.insert(1);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_size() {
        let mut list = LinkedList::new();
        assert_eq!(list.size(), 0);

        list.insert(1);
        list.insert(2);
        assert_eq!(list.size(), 2);

        list.delete(&1).unwrap();
        assert_eq!(list.size(), 1);
    }
}