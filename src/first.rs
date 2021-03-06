use std::mem;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List{head: Link::Empty}
    }
}

impl List {
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }




}


impl List {
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                let node = *node;
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut l = List::new();
        assert_eq!(l.pop(), None);

        l.push(1);
        l.push(2);
        l.push(3);

        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));

        l.push(4);
        l.push(5);

        assert_eq!(l.pop(), Some(5));
        assert_eq!(l.pop(), Some(4));
        assert_eq!(l.pop(), Some(1));
    }
}