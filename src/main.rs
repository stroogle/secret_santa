mod graph;
mod person;

pub use crate::graph::Graph;

pub use crate::person::Person;

fn main() {
    println!("Hello, world!");
    // let x = Graph::new(9);
    // println!("graph x is of size {}", x.get_size());
    let people = vec![Person::new("bruh moment", "123456"), Person::new("John Doe", "234234243"), Person::new("me", "bruh")];
    let my_graph = Graph::new(people);
    println!("graph my_graph is of size {}", my_graph.get_size());
}