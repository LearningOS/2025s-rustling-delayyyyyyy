/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        
		let n = self.adj.len();
        if n == 0 {
            return vec![]; // 如果图为空，返回空列表
        }

        let mut visited = vec![false; n]; // 记录哪些节点已被访问
        let mut queue = VecDeque::new(); // 创建一个队列
        let mut visit_order = vec![]; // 存储访问顺序

        // 初始化：将起始节点加入队列，并标记为已访问
        visited[start] = true;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() { // 当队列不为空时
            visit_order.push(node); // 记录当前节点的访问顺序

            // 遍历当前节点的所有邻接节点
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] { // 如果邻接节点未被访问过
                    visited[neighbor] = true; // 标记为已访问
                    queue.push_back(neighbor); // 加入队列
                }
            }
        }

        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

