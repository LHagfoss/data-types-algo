fn size() {
    return size;
}

fn toArray() {}

pub fn reverse() {
    let previous = first;
    let current = first.next;

    while current != null {
        let next = current.next;
        current.next = previous;
        previous = current;
        current = next;
    }

    last = first;
    last.next = null;
    first = previous;
}
