use crate::structures::directed_graph::DirectedGraph;

pub fn dfs(graph: DirectedGraph, start_node: i32) -> Vec<i32> {
    if !graph.contains_node(start_node) {
        panic!("'start_node' not found in the provided graph");
    }
    let mut stack = vec![];
    let mut visited = vec![];
    stack.push(start_node);
    while !stack.is_empty() {
        let current_node = stack.pop().unwrap();
        let mut neighbors = graph.get_neighbors(current_node);
        stack.append(&mut neighbors);
        visited.push(current_node);
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::dfs;
    use crate::structures::directed_graph::DirectedGraph;

    #[test]
    fn it_performs_dfs() {
        let mut graph = DirectedGraph::new();
        graph
            .add_edge(1, 2)
            .add_edge(1, 3)
            .add_edge(2, 4)
            .add_edge(2, 5);

        let result = dfs(graph, 1);
        assert_eq!(result, vec![1, 3, 2, 5, 4]);
    }

    #[test]
    #[should_panic(expected = "'start_node' not found in the provided graph")]
    fn it_performs_dfs_with_invalid_start_node() {
        let mut graph = DirectedGraph::new();
        graph
            .add_edge(1, 2)
            .add_edge(1, 3)
            .add_edge(2, 4)
            .add_edge(2, 5);

        dfs(graph, 6);
    }
}
