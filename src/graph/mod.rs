pub struct Graph<T> {
    size: i32,
    body: Vec<Vec<i32>>,
}

impl Graph<T> {
    pub fn new(size: i32) -> Self {
        let mut g = Graph {
            size,
            body: vec![vec![1; size.try_into().unwrap()]; size.try_into().unwrap()],
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

    fn fix_connections(g: &mut Graph) {
        for n in 0..g.size {
            g.body[usize::try_from(n).unwrap()][usize::try_from(n).unwrap()] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_graph() {
        let g = Graph::new(8);
        assert_eq!(g.get_size(), 8);
    }

    #[test]
    fn get_body() {
        let g = Graph::new(8);
        assert_eq!(g.get_body()[0][0], 0);

        assert_eq!(g.get_body()[6][6], 0);

        assert_eq!(g.get_body()[0][4], 1);
    }
}