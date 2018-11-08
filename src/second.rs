use std::mem;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List{head: None}
    }
}

impl<T> List<T> {
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }




}


impl<T> List<T> {
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;
            node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut l = List::new();
        assert_eq!(l.pop(), None);

        l.push("1");
        l.push("2");
        l.push("3");

        assert_eq!(l.pop(), Some("3"));
        assert_eq!(l.pop(), Some("2"));

        l.push("4");
        l.push("5");

        assert_eq!(l.pop(), Some("5"));
        assert_eq!(l.pop(), Some("4"));
        assert_eq!(l.pop(), Some("1"));
    }
}