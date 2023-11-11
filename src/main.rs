mod graph;

pub use crate::graph::Graph; 

fn main() {
    println!("Hello, world!");
    let x = Graph::new(9);
    println!("graph x is of size {}", x.get_size());
}