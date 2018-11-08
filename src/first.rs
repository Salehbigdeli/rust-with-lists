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
    fn push(&mut self, item: i32) {
        let new_node = Box::new(Node {
            elem: item,
            next: mem::replace(&mut self.head, Link::Empty)
        });
    }
}


impl List {
    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
        Link::Empty => None,
        Link::More(boxed_node) => {
            let node = *boxed_node;
            self.head = node.next;
            Some(node.elem)
            }
        }
    }
}

// impl List {
//     pub fn add(&mut self, item: i32) -> List{
//         match self {
//             List::Empty => {
//                 self = &mut List::Elem(item, Box::new(List::Empty))
//             },
//             List::Elem(_i, _next) => List::Empty,
//         }
//     }
// }