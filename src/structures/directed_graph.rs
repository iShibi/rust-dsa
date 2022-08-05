use std::collections::HashMap;

pub struct DirectedGraph {
    _map: HashMap<i32, Vec<i32>>,
}

impl DirectedGraph {
    pub fn new() -> DirectedGraph {
        return DirectedGraph {
            _map: HashMap::new(),
        };
    }
}

impl DirectedGraph {
    pub fn add_edge(&mut self, src_node: i32, des_node: i32) -> &mut DirectedGraph {
        if let Some(v) = self._map.get_mut(&src_node) {
            v.push(des_node);
        } else {
            self._map.insert(src_node, vec![des_node]);
        }

        if let None = self._map.get_mut(&des_node) {
            self._map.insert(des_node, vec![]);
        }
        self
    }

    pub fn get_neighbors(&self, src_node: i32) -> Vec<i32> {
        match self._map.get(&src_node) {
            Some(v) => v.to_vec(),
            None => {
                vec![]
            }
        }
    }

    pub fn contains_node(&self, node: i32) -> bool {
        return self._map.contains_key(&node);
    }
}

#[cfg(test)]
mod tests {
    use super::DirectedGraph;
    use std::collections::HashMap;

    #[test]
    fn it_adds_edges() {
        let mut graph = DirectedGraph::new();
        graph
            .add_edge(1, 2)
            .add_edge(1, 3)
            .add_edge(2, 4)
            .add_edge(2, 5);

        let mut map = HashMap::new();
        map.insert(1, vec![2, 3]);
        map.insert(2, vec![4, 5]);
        map.insert(3, vec![]);
        map.insert(4, vec![]);
        map.insert(5, vec![]);

        assert_eq!(graph._map, map);
    }

    #[test]
    fn it_gets_neighbors() {
        let mut graph = DirectedGraph::new();
        graph
            .add_edge(1, 2)
            .add_edge(1, 3)
            .add_edge(2, 4)
            .add_edge(2, 5);

        assert_eq!(graph.get_neighbors(1), vec![2, 3]);
        assert_eq!(graph.get_neighbors(2), vec![4, 5]);
        assert_eq!(graph.get_neighbors(3), vec![]);
    }

    #[test]
    fn it_contains_node() {
        let mut graph = DirectedGraph::new();
        graph
            .add_edge(1, 2)
            .add_edge(1, 3)
            .add_edge(2, 4)
            .add_edge(2, 5);

        assert_eq!(graph.contains_node(1), true);
        assert_eq!(graph.contains_node(6), false);
    }
}
