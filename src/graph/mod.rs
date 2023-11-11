pub struct Graph<T> {
    size: i32,
    body: Vec<Vec<i32>>,
    items: Vec<T>
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

    fn fix_connections(g: &mut Graph<T>) {
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
}