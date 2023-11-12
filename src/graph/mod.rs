pub mod hamiltonian_cycle;

pub struct Graph<T> {
    size: i32,
    body: Vec<Vec<i32>>,
    items: Vec<T>,
}

impl<T> Graph<T> {
    pub fn new(items: Vec<T>) -> Self {
        let mut g = Graph {
            size: items.len() as i32,
            body: vec![vec![1; items.len().try_into().unwrap()]; items.len().try_into().unwrap()],
            items,
        };

        Graph::fix_connections(&mut g);

        g
    }

    pub fn get_size(&self) -> i32 {
        self.size
    }

    pub fn get_body(&self) -> &Vec<Vec<i32>> {
        &self.body
    }

    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn remove_edge(&mut self, vertex_a: i32, vertex_b: i32) -> Result<EdgeStatus, String> {
        let max_vertex = (self.body.len() - 1) as i32;
        match vertex_a > max_vertex || vertex_b > max_vertex {
            true => Err(format!("vertex_a: {}, vertex_b: {}, maximum valid vertex {}", vertex_a, vertex_b, self.body.len() - 1)),
            _ => {
                self.body[usize::try_from(vertex_a).unwrap()][usize::try_from(vertex_b).unwrap()] = 0;
                self.body[usize::try_from(vertex_b).unwrap()][usize::try_from(vertex_a).unwrap()] = 0;
                Ok(EdgeStatus::Removed)
            }
        }
    }

    pub fn add_edge(&mut self, vertex_a: i32, vertex_b: i32) -> Result<EdgeStatus, String> {
        let max_vertex = (self.body.len() - 1) as i32;
        match vertex_a > max_vertex || vertex_b > max_vertex {
            true => Err(format!("vertex_a: {}, vertex_b: {}, maximum valid vertex {}", vertex_a, vertex_b, self.body.len() - 1)),
            _ => {
                self.body[usize::try_from(vertex_a).unwrap()][usize::try_from(vertex_b).unwrap()] = 1;
                self.body[usize::try_from(vertex_b).unwrap()][usize::try_from(vertex_a).unwrap()] = 1;
                Ok(EdgeStatus::Added)
            }
        }
        
    }

    fn fix_connections(g: &mut Graph<T>) {
        for n in 0..g.size {
            g.body[usize::try_from(n).unwrap()][usize::try_from(n).unwrap()] = 0;
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum EdgeStatus { Removed, Added }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph() {
        let g = Graph::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(g.get_size(), 8);
    }

    #[test]
    fn get_body() {
        let g = Graph::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(g.get_body()[0][0], 0);

        assert_eq!(g.get_body()[6][6], 0);

        assert_eq!(g.get_body()[0][4], 1);
    }

    #[test]
    fn get_items() {
        let g = Graph::new(vec![0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(g.get_items()[3], 3);
    }

    #[test]
    fn add_edge() {
        let mut g = Graph::new(vec![0, 1, 2]);
        assert_eq!(g.get_body()[0][0], 0);
        assert_eq!(g.add_edge(0, 0).unwrap(), EdgeStatus::Added);
        assert_eq!(g.get_body()[0][0], 1);

        assert_eq!(g.add_edge(4, 7), Err(String::from("vertex_a: 4, vertex_b: 7, maximum valid vertex 2")));
    }

    #[test]
    fn remove_edge() {
        let mut g = Graph::new(vec![0, 1, 2]);
        assert_eq!(g.get_body()[0][1], 1);
        assert_eq!(g.get_body()[1][0], 1);
        assert_eq!(g.remove_edge(0, 1).unwrap(), EdgeStatus::Removed);
        assert_eq!(g.get_body()[0][1], 0);
        assert_eq!(g.get_body()[1][0], 0);

        assert_eq!(g.remove_edge(4, 7), Err(String::from("vertex_a: 4, vertex_b: 7, maximum valid vertex 2")));
    }
}