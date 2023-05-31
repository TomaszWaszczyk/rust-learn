mod linked_list {

    pub struct LinkedList<T> {
        head: Option<Node<T>>,
    }

    struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T> LinkedList<T> {
        fn new(value: T) -> Self {
            LinkedList{ head: None }
        }

        fn push(&mut self, _value: T) {
            let prev_head = self.head.take();

            let new_head = match prev_head {
                None => Node { value: _value, next: None },
                Some(prev_head) => Node { value: _value, next: Some(Box::new(prev_head)) }
            };

            self.head = Some(new_head);
        }

        fn pop(&mut self) -> Option<&T> {
            let prev_head = self.head.take();

            let new_head = match prev_head {
                None => _,
                Some(node) => Some(*node)
            };

            self.head = new_head;
            Some(&old_head.value);
        }
    }
}
