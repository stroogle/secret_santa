mod graph;
mod person;

pub use crate::graph::Graph;

pub use crate::graph::hamiltonian_cycle::HamiltonianCycle;

use rand::seq::SliceRandom;
use rand::thread_rng;

pub use crate::person::Person;

fn main() {

    let mut people = vec![Person::new("John Doe", "123456"), Person::new("Jane Bruh", "234234243"), Person::new("Thomas Edison", "bruh")];
    let mut rng = thread_rng();

    people.shuffle(&mut rng);
    
    let my_graph = Graph::new(people.clone());
    let path = HamiltonianCycle::new(&my_graph);


    match path {
        Some(x) => {
            println!("{:?}", x);
            for n in 0..x.len() - 1 {
                let first_person = &my_graph.get_items()[usize::try_from(x[usize::try_from(n).unwrap()]).unwrap()];
                let second_person = &my_graph.get_items()[usize::try_from(x[usize::try_from(n + 1).unwrap()]).unwrap()];
                println!("{} -> {}", first_person.get_name(), second_person.get_name());
            }
        },
        None => println!("No path found :(")
    }
}