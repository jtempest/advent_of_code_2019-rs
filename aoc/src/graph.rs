pub trait Graph {
    fn num_nodes(&self) -> usize;
    fn node_edges(&self, node_index: usize) -> Vec<Edge>;

    fn shortest_path_search(
        &self,
        start_index: usize,
        dest_index: Option<usize>,
    ) -> PathSearchResult {
        let num_nodes = self.num_nodes();

        let mut previous_node = Vec::new();
        previous_node.resize(num_nodes, None);

        let mut costs = Vec::new();
        costs.resize(num_nodes, None);

        let mut open = Vec::new();
        open.push((None, start_index, 0));

        let mut num_found = 0;

        while let Some((prev, node, cost)) = open.pop() {
            previous_node[node] = prev;
            costs[node] = Some(cost);
            num_found += 1;

            // If we filled the whole graph or reached our destination, we're done.
            if num_found == num_nodes || Some(node) == dest_index {
                break;
            }

            for e in self.node_edges(node).into_iter() {
                let next = e.dest_index;
                if costs[next].is_none() {
                    open.push((Some(node), next, cost + e.cost));
                }
            }
            open.sort_by(|a, b| a.2.cmp(&b.2).reverse());
        }

        PathSearchResult {
            start_index,
            dest_index,
            previous_node,
            costs,
        }
    }

    fn find_shortest_path_indices(
        &self,
        start_index: usize,
        dest_index: usize,
    ) -> Option<Vec<usize>> {
        self.shortest_path_search(start_index, Some(dest_index))
            .make_path()
    }

    fn farthest_distance_from(&self, start_index: usize) -> usize {
        self.shortest_path_search(start_index, None).highest_cost()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Edge {
    pub dest_index: usize,
    pub cost: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PathSearchResult {
    pub start_index: usize,
    pub dest_index: Option<usize>,
    pub previous_node: Vec<Option<usize>>,
    pub costs: Vec<Option<usize>>,
}

impl PathSearchResult {
    pub fn make_path(&self) -> Option<Vec<usize>> {
        let mut index = self.dest_index?;
        let mut path = vec![index];
        while let Some(next) = self.previous_node[index] {
            path.push(next);
            index = next;
        }
        path.reverse();
        Some(path)
    }

    pub fn highest_cost(&self) -> usize {
        self.costs.iter().max().unwrap().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;

    struct TestGraph {
        nodes: Vec<u8>,
        edges: HashMap<u8, Vec<u8>>,
    }

    impl Graph for TestGraph {
        fn num_nodes(&self) -> usize {
            self.nodes.len()
        }

        fn node_edges(&self, node_index: usize) -> Vec<Edge> {
            let node = self.nodes[node_index];
            self.edges[&node]
                .iter()
                .map(|&dest| Edge {
                    dest_index: dest as usize,
                    cost: 1,
                })
                .collect()
        }
    }

    fn make_graph() -> TestGraph {
        /*
            1----0    4
              \  |  / |
               \ | /  |
                 3----2
        */

        let nodes = (0..5).collect();

        let mut edges = HashMap::new();
        edges.insert(0, vec![1, 3]);
        edges.insert(1, vec![0, 3]);
        edges.insert(2, vec![3, 4]);
        edges.insert(3, vec![0, 1, 2, 4]);
        edges.insert(4, vec![2, 3]);

        TestGraph { nodes, edges }
    }

    #[test]
    fn test_graph() {
        let graph = make_graph();

        assert_eq!(graph.num_nodes(), 5);

        let from3 = graph.node_edges(3);
        assert_eq!(from3.len(), 4);
        assert!(from3.contains(&Edge {
            dest_index: 0,
            cost: 1
        }));
        assert!(from3.contains(&Edge {
            dest_index: 1,
            cost: 1
        }));
        assert!(from3.contains(&Edge {
            dest_index: 2,
            cost: 1
        }));
        assert!(from3.contains(&Edge {
            dest_index: 4,
            cost: 1
        }));
    }

    #[test]
    fn test_shortest_path() {
        let graph = make_graph();

        let info = graph.shortest_path_search(0, None);
        assert_eq!(
            info.previous_node,
            vec![None, Some(0), Some(3), Some(0), Some(3)]
        );
        assert_eq!(
            info.costs,
            vec![Some(0), Some(1), Some(2), Some(1), Some(2)]
        );

        let path = graph.find_shortest_path_indices(4, 1).unwrap();
        assert_eq!(path, vec![4, 3, 1]);

        let dist = graph.farthest_distance_from(4);
        assert_eq!(dist, 2);
    }
}
