use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl UndirectedGraph {
    pub fn new() -> Self {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    // 添加节点（确保节点存在）
    fn add_node(&mut self, node: &str) -> bool {
        let node_str = node.to_string();
        self.adjacency_table_mutable().entry(node_str).or_insert_with(Vec::new);
        true // 始终返回true，表示成功添加（即使已存在）
    }

    // 添加边（核心实现）
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (from, to, weight) = edge;
        let from_str = from.to_string();
        let to_str = to.to_string();

        // 确保两个节点存在
        self.add_node(from);
        self.add_node(to);

        // 添加边到from的邻接表
        self.adjacency_table_mutable()
            .get_mut(from)
            .unwrap()
            .push((to_str.clone(), weight));

        // 添加边到to的邻接表（反向边）
        self.adjacency_table_mutable()
            .get_mut(to)
            .unwrap()
            .push((from_str.clone(), weight));
    }

    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, neighbors) in self.adjacency_table() {
            for (to_node, weight) in neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

impl Graph for UndirectedGraph {
    fn new() -> Self {
        UndirectedGraph::new()
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&"a".into(), &"b".into(), 5),
            (&"b".into(), &"a".into(), 5),
            (&"b".into(), &"c".into(), 10),
            (&"c".into(), &"b".into(), 10),
            (&"c".into(), &"a".into(), 7),
            (&"a".into(), &"c".into(), 7),
        ];

        for edge in &expected_edges {
            assert!(graph.edges().contains(edge), "Missing edge: {:?}", edge);
        }
    }
}