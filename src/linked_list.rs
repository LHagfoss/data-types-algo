struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    let Node: first;
    let Node: last;

    fn new() {
        LinkedList { head: None }
    }

    fn add_last(item: i32) {
        let node = Node(item);
    }
}
