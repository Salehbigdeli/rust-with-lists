extern crate list;
use list::first::List;

fn main() {
    let l: List = List::Elem(12, Box::new(List::Elem(13, Box::new(List::Empty))));
   println!("{:?}", l);
}                             