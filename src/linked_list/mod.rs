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
    }
}
