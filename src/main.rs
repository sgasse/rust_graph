// Copyright 2021 Simon B. Gasse

use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
/// Structure representing a graph with String as node names
struct Graph {
    names: Vec<Box<String>>,
    name2idx: HashMap<String, usize>,
    parents: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
}

/// Trait containing node buffers used during search
/// 
/// Abstracts a stack and a queue for graph traversal
trait SearchBuffer<T> {
    fn enlist(&mut self, idx: T);
    fn get_next(&mut self) -> Option<T>;
    fn is_empty(&self) -> bool;
}

impl<T> SearchBuffer<T> for Vec<T> {
    fn enlist(&mut self, val: T) {
        self.push(val);
    }
    fn get_next(&mut self) -> Option<T> {
        self.pop()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T> SearchBuffer<T> for VecDeque<T> {
    fn enlist(&mut self, val: T) {
        self.push_back(val);
    }
    fn get_next(&mut self) -> Option<T> {
        self.pop_front()
    }
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl Graph {
    /// Create a new empty graph
    fn new() -> Graph {
        Graph { names: Vec::new(), parents: Vec::new(), children: Vec::new(), name2idx: HashMap::new() }
    }

    /// Create graph from file
    fn from_file(filename: &str) -> Graph {
        println!("Creating graph from file: {}", filename);

        let mut graph = Graph::new();

        let contents = fs::read_to_string(filename).expect("Could not read file");
        for (i, line) in contents.lines().enumerate() {
            let (parent, children) = Graph::parse_line(line);
            println!("Line {} | Parent: {}, Children: {:?}", i, parent, children);
            let _ = graph.add_with_children(parent, children);
        }

        graph
    }

    /// Add a node with its children to the graph
    fn add_with_children_r(&mut self, name: &str, children: Vec<&str>) {
        let name_ = name.to_string();
        let children_: Vec<_> = children.iter().map(|x| { x.to_string() }).collect();
        self.add_with_children(name_, children_);
    }

    /// Runs a depth-first-search on the graph
    fn dfs(&self, start: String) -> Result<Vec<String>, &str> {
        let mut stack: Vec<usize> = Vec::new();
        self.traverse(start, &mut stack)
    }

    /// Runs a breadth-first-search on the graph
    fn bfs(&self, start: String) -> Result<Vec<String>, &str> {
        let mut queue: VecDeque<usize> = VecDeque::new();
        self.traverse(start, &mut queue)
    }

    /// Parse a line for creating a graph from file
    fn parse_line(line: &str) -> (String, Vec<String>) {
        let parts: Vec<&str> = line.split("->").collect();
        let children: Vec<&str> = parts[1].split(",").collect();
        (parts[0].to_string(), children.iter().map(|x| x.to_string()).collect())
    }

    /// Add a node with its children to the graph
    fn add_with_children(&mut self, name: String, children: Vec<String>) {
        let node_idx = self.parents.len();

        // Check if parent is already there
        if !self.name2idx.contains_key(&name.clone()) {
            // Parent node does not exist - create
            self.names.push(Box::new(name.clone()));
            self.name2idx.insert(name.clone(), node_idx);
            self.parents.push(None);
            self.children.push(Vec::new());
        }

        // Add children
        for child in children.iter() {
            let _ = self.add_to_parent((*child).clone(), name.clone());
        }
    }

    /// Add a child node to an existing parent node
    fn add_to_parent(&mut self, child: String, parent: String) -> Result<usize, &str> {
        // Parent node exists
        let p_idx = *(self.name2idx.get(&parent).unwrap());

        let node_idx = self.parents.len();

        // Check if child node already exists
        if self.name2idx.contains_key(&child.clone()) {
            return Err("Node exists already");
        }

        self.names.push(Box::new(child.clone()));
        self.name2idx.insert(child.clone(), node_idx);
        self.parents.push(Some(p_idx));
        self.children.push(Vec::new());
        self.children[p_idx].push(node_idx);
        Ok(node_idx)
    }

    /// Walks the graph with a predefined buffer
    /// 
    /// # Arguments
    /// 
    /// * `start` - A String defining the name of the start node for traversal
    /// * `buffer` - A SearchBuffer, e.g. a stack for DFS or a queue for BFS
    fn traverse(&self, start: String, buffer: &mut dyn SearchBuffer<usize>) -> Result<Vec<String>, &str> {
        let mut visited: HashSet<usize> = HashSet::new();

        let mut traversal: Vec<String> = Vec::new();

        match self.name2idx.get(&start) {
            Some(&start_idx) => buffer.enlist(start_idx),
            None => return Err("Start node not found."),
        }

        while !buffer.is_empty() {
            // Get next node from the buffer
            let node_idx = buffer.get_next().unwrap();

            // Add children if they are not yet visited
            for child in &self.children[node_idx] {
                if !visited.contains(child) {
                    buffer.enlist(*child);
                }
            }

            // Add current node to traversal
            let node_name = (*self.names[node_idx]).clone();
            traversal.push(node_name);

            // Mark current node as visited
            visited.insert(node_idx);
        }

        Ok(traversal)

    }

}


fn main() {
    println!("A graph of nodes in Rust!");

    let mut gr = Graph::new();
    let _ = gr.add_with_children_r("A", vec!["B", "C", "D"]);

    println!("\n{:?}\n", &gr);

    let gr_from_file = Graph::from_file("graph.txt");
    println!("\n{:?}\n", gr_from_file);

    let mut traversal = gr_from_file.dfs("A".to_string());
    println!("DFS:\n{:?}", traversal);

    traversal = gr_from_file.bfs("A".to_string());
    println!("BFS:\n{:?}", traversal);
}


#[cfg(test)]
mod test {

    use super::Graph;

    fn assert_graph_field_len(graph: &Graph, len: usize) {
        assert_eq!(graph.names.len(), len);
        assert_eq!(graph.parents.len(), len);
        assert_eq!(graph.children.len(), len);
    }

    #[test]
    fn using_box_vec() {
        let bv = vec![Box::new("a")];
        assert_eq!(bv.contains(&Box::new("a")), true);
        assert_eq!(bv.contains(&Box::new("b")), false);
    }

    #[test]
    fn test_graph_new() {
        let gr = Graph::new();
        assert_graph_field_len(&gr, 0);
    }

    #[test]
    fn test_add_to_parent() {
        let mut gr = Graph::new();

        // Add a node without parent
        gr.add_with_children_r("A", Vec::new());

        // Add children to root node
        let _ = gr.add_to_parent("B".to_string(), "A".to_string());
        assert_graph_field_len(&gr, 2);
        assert_eq!(gr.children[0].len(), 1);

        // Reject existing node
        let mut res = gr.add_to_parent("A".to_string(), "B".to_string());
        assert_eq!(res, Err("Node exists already"));

        // Add node with exotic name
        res = gr.add_to_parent("Wéïrd näµëß§".to_string(), "A".to_string());
        assert_eq!(res, Ok(2));
        assert_graph_field_len(&gr, 3);
    }

    #[test]
    fn test_add_with_children() {
        let mut gr = Graph::new();

        // Add node without parent
        gr.add_with_children_r("Root", Vec::new());
        assert_graph_field_len(&gr, 1);

        // Add children to existing parent
        gr.add_with_children_r("Root", vec!["A"]);
        assert_graph_field_len(&gr, 2);

        // Add more children
        let parent = "A";
        let children = vec!["B", "C", "D"];
        gr.add_with_children_r(parent, children);

        assert_graph_field_len(&gr, 5);
        assert_eq!(gr.children[1].len(), 3);
    }

    #[test]
    fn test_parse_line() {
        let l1 = "A->B,C";
        let (parent1, children1) = Graph::parse_line(l1);
        let ref_children = vec!["B".to_string(), "C".to_string()];
        assert_eq!(parent1, "A".to_string());
        assert_eq!(children1, ref_children);
    }
}
