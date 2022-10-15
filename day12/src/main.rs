//#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;

const FILENAME: &str = "input.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_data = read_to_string(FILENAME)?;
    let graph = Graph::from_string(&file_data);
    println!("Part 1 : {}", graph.path_iter::<FastRoute>().count());
    println!("Part 2 : {}", graph.path_iter::<ScenicRoute>().count());
    Ok(())
}

#[derive(Debug)]
struct Node {
    name: String,
    is_big: bool,
    is_end: bool,
    is_start: bool,
    edges: Vec<NodeIndex>,
}

impl Node {
    pub fn new(name: &str) -> Self {
        Node {
            name: name.to_owned(),
            is_big: name.chars().all(char::is_uppercase),
            is_end: name == "end",
            is_start: name == "start",
            edges: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct NodeIndex(usize);

impl NodeIndex {
    fn new(index: usize) -> Self {
        Self(index)
    }
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Default, Debug)]
struct Graph {
    name_to_node_index: HashMap<String, NodeIndex>,
    nodes: Vec<Node>,
}

impl Graph {
    pub fn from_string(text: &str) -> Graph {
        let mut result = Graph::default();
        text.trim().lines().for_each(|line| {
            let mut parts = line.split('-');
            result.new_edge(parts.next().unwrap(), parts.next().unwrap());
        });
        result
    }

    pub fn ensure_node_name(&mut self, name: &str) -> NodeIndex {
        if let Some(node_index) = self.name_to_node_index.get(name) {
            *node_index
        } else {
            let result = NodeIndex::new(self.nodes.len());
            let node = Node::new(name);
            self.name_to_node_index.insert(node.name.clone(), result);
            self.nodes.push(node);
            result
        }
    }

    pub fn node_mut(&mut self, node_index: NodeIndex) -> &mut Node {
        self.nodes.get_mut(node_index.index()).unwrap()
    }

    pub fn node(&self, node_index: NodeIndex) -> &Node {
        self.nodes.get(node_index.index()).unwrap()
    }

    pub fn node_index_by_name(&self, name: &str) -> Option<NodeIndex> {
        self.name_to_node_index.get(name).copied()
    }

    #[cfg(test)]
    pub fn node_by_name(&self, name: &str) -> Option<&Node> {
        self.node_index_by_name(name)
            .and_then(|idx| self.nodes.get(idx.index()))
    }

    pub fn new_edge(&mut self, from_name: &str, to_name: &str) {
        let (from_idx, to_idx) = (
            self.ensure_node_name(from_name),
            self.ensure_node_name(to_name),
        );
        self.node_mut(from_idx).edges.push(to_idx);
        self.node_mut(to_idx).edges.push(from_idx);
    }

    pub fn path_iter<Sr: Route + Default + 'static>(
        &self,
    ) -> impl Iterator<Item = Vec<NodeIndex>> + '_ {
        PathIterator {
            graph: self,
            stack: vec![],
            route: Sr::default(),
            current_path: vec![],
            start: self.node_index_by_name("start").unwrap(),
            done: false,
        }
    }

    #[allow(dead_code)]
    pub fn path_to_string(&self, path: &[NodeIndex]) -> String {
        path.iter()
            .map(|&id| self.node(id).name.to_owned())
            .collect::<Vec<String>>()
            .join(",")
    }
}

trait Route {
    fn insert(&mut self, index: NodeIndex);
    fn contains(&self, index: &NodeIndex) -> bool;
    fn remove(&mut self, index: &NodeIndex);
}

impl Route for FastRoute {
    fn insert(&mut self, index: NodeIndex) {
        self.insert(index);
    }

    fn contains(&self, index: &NodeIndex) -> bool {
        self.contains(index)
    }

    fn remove(&mut self, index: &NodeIndex) {
        self.remove(index);
    }
}

type FastRoute = HashSet<NodeIndex>;

#[derive(Debug, Default)]
struct ScenicRoute {
    twice_visited: Option<NodeIndex>,
    fast_route: FastRoute,
}

impl Route for ScenicRoute {
    fn insert(&mut self, index: NodeIndex) {
        if self.fast_route.contains(&index) {
            if Some(index) == self.twice_visited {
                panic!("already twice visited !? {:?}", index);
            } else {
                self.twice_visited = Some(index)
            }
        } else {
            self.fast_route.insert(index);
        }
    }

    fn contains(&self, index: &NodeIndex) -> bool {
        self.fast_route.contains(index) && self.twice_visited.is_some()
    }

    fn remove(&mut self, index: &NodeIndex) {
        if Some(*index) == self.twice_visited {
            self.twice_visited = None;
        } else {
            self.fast_route.remove(index);
        }
    }
}

struct PathIterator<'a, Sr: Route> {
    graph: &'a Graph,
    stack: Vec<&'a [NodeIndex]>,
    route: Sr,
    current_path: Vec<NodeIndex>,
    start: NodeIndex,
    done: bool,
}

impl<'a, Sr: Route> Iterator for PathIterator<'a, Sr> {
    type Item = Vec<NodeIndex>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            if self.current_path.is_empty() {
                self.route.insert(self.start);
                self.current_path.push(self.start);
                self.stack.push(&self.graph.node(self.start).edges);
            }
            while let Some(stack_frame) = self.stack.pop() {
                if let Some((first, rest)) = stack_frame.split_first() {
                    self.stack.push(rest);
                    if self.push_node(*first) {
                        if self.graph.node(*first).is_end {
                            return Some(self.current_path.clone());
                        } else {
                            // not the end, we continue with the pushed stack frame
                            continue;
                        }
                    } else {
                        // couldn't push the node, this is a dead_end, we continue with current stack frame
                        continue;
                    }
                } else {
                    // we exhausted the stack frame, time to backtrack
                    // stackframe is already poped, only need to remove last path node
                    if let Some(index) = self.current_path.pop() {
                        self.route.remove(&index);
                    }
                }
            }
            // there are no more stack frames, we're done here
            self.done = true;
            None
        }
    }
}

impl<'a, Sr: Route> PathIterator<'a, Sr> {
    fn push_node(&mut self, node_id: NodeIndex) -> bool {
        let node = self.graph.node(node_id);
        if node.is_start || self.route.contains(&node_id) {
            false
        } else {
            if !node.is_big {
                self.route.insert(node_id);
            }
            self.current_path.push(node_id);
            self.stack.push(if node.is_end { &[] } else { &node.edges });
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn test_is_big() {
        assert!(!Node::new("bAe").is_big);
        assert!(Node::new("BAE").is_big);
        assert_eq!(size_of::<NodeIndex>(), size_of::<usize>()); // 64 bits, just the usize
    }

    #[test]
    fn test_graph_from_string() {
        let graph = Graph::from_string(EXAMPLE1);
        assert_eq!(graph.nodes.len(), 6);
        assert_eq!(graph.name_to_node_index.len(), 6);
        assert_eq!(graph.node_by_name("start").unwrap().edges.len(), 2);
        assert_eq!(graph.node_by_name("A").unwrap().edges.len(), 4);
    }

    #[test]
    fn test_path_iterator() {
        let graph = Graph::from_string(EXAMPLE1);

        // for path in graph.path_iter::<ScenicRoute>() {
        //     println!("{:?}", graph.path_to_string(&path));
        // }
        // assert!(false);

        assert_eq!(graph.path_iter::<FastRoute>().count(), 10);
        let graph = Graph::from_string(EXAMPLE2);
        assert_eq!(graph.path_iter::<FastRoute>().count(), 19);
        let graph = Graph::from_string(EXAMPLE3);
        assert_eq!(graph.path_iter::<FastRoute>().count(), 226);
    }

    #[test]
    fn test_scenice_route_path_iterator() {
        let graph = Graph::from_string(EXAMPLE1);
        assert_eq!(graph.path_iter::<ScenicRoute>().count(), 36);
        let graph = Graph::from_string(EXAMPLE2);
        assert_eq!(graph.path_iter::<ScenicRoute>().count(), 103);
        let graph = Graph::from_string(EXAMPLE3);
        assert_eq!(graph.path_iter::<ScenicRoute>().count(), 3509);
    }

    const EXAMPLE1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const EXAMPLE2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const EXAMPLE3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
}
