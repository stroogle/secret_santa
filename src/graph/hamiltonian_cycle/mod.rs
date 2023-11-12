pub use crate::graph::Graph;

pub struct HamiltonianCycle<'a, T> {
    path: Vec<i32>,
    graph: &'a Graph<T>,
}

impl<'a, T> HamiltonianCycle<'_, T> {
    pub fn new(graph: &'a Graph<T>) -> Option<Vec<i32>> {
        if graph.get_size() < 3 {
            return None;
        }

        let mut cycle = HamiltonianCycle {
            path: Vec::<i32>::new(),
            graph
        };

        cycle.find_path(0);

        let cycle_size = cycle.path.len() as i32;
        let expected_cycle_size = cycle.graph.get_size() + 1;

        if cycle_size == expected_cycle_size {
            Some(cycle.path)
        } else {
            None
        }
    }

    pub fn find_path(&mut self, source_vertex: i32) {
        if self.path_is_complete() {
            return;
        }

        self.path.push(source_vertex);

        for destination_vertex in 0..self.graph.get_size() {
            let destination_seen = self.path.contains(&destination_vertex);
            let edge_to_destination = self.graph.body[usize::try_from(source_vertex).unwrap()][usize::try_from(destination_vertex).unwrap()] == 1;

            if !destination_seen && edge_to_destination {
                self.find_path(destination_vertex);
            }
        }

        if self.path.len() as i32 == self.graph.get_size() && self.graph.body[usize::try_from(source_vertex).unwrap()][0] == 1 {
            self.path.push(0);
        }

        if self.path_is_not_complete() {
            self.path.pop();
        }
    }

    pub fn path_is_complete(&self) -> bool {
        self.path.len() as i32 == self.graph.get_size() + 1
    }

    pub fn path_is_not_complete(&self) -> bool {
        !self.path_is_complete()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_hamiltonian_path() {
        let mut g = Graph::new(vec![0, 1, 2]);

        let x = HamiltonianCycle::new(&g);

        assert_eq!(x, Some(Vec::from([0, 1, 2, 0])));

        let _ = g.remove_edge(0, 1);

        let y = HamiltonianCycle::new(&g);

        assert_eq!(y, None);

        let _ = g.add_edge(0, 1);

        let z = HamiltonianCycle::new(&g);

        assert_eq!(z, Some(Vec::from([0, 1, 2, 0])));

        let _ = g.remove_edge(0, 1);
        let _ = g.remove_edge(0, 2);

        let disconnected_graph_path = HamiltonianCycle::new(&g);

        assert_eq!(disconnected_graph_path, None);
    }
}