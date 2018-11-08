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

pub struct List {
    head: Link,
}

// impl List {
//     pub fn new() -> List {
//         List::Empty
//     }
// }

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