// Copyright 2021 Simon B. Gasse

use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::trees::common::SearchBuffer;

#[derive(Debug)]
/// Structure representing a look-up tree with String as node names
pub struct NamedLuTree {
    names: Vec<Box<String>>,
    name2idx: HashMap<String, usize>,
    parents: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
}

impl NamedLuTree {
    /// Create a new empty look-up tree
    pub fn new() -> NamedLuTree {
        NamedLuTree { names: Vec::new(), parents: Vec::new(), children: Vec::new(), name2idx: HashMap::new() }
    }

    /// Create look-up tree from file
    pub fn from_file(filename: &str) -> NamedLuTree {
        println!("Creating look-up tree from file: {}", filename);

        let mut lutree = NamedLuTree::new();

        let contents = fs::read_to_string(filename).expect("Could not read file");
        for (i, line) in contents.lines().enumerate() {
            let (parent, children) = NamedLuTree::parse_line(line);
            println!("Line {} | Parent: {}, Children: {:?}", i, parent, children);
            let _ = lutree.add_with_children(parent, children);
        }

        lutree
    }

    /// Add a node with its children to the look-up tree
    pub fn add_with_children_r(&mut self, name: &str, children: Vec<&str>) {
        let name_ = name.to_string();
        let children_: Vec<_> = children.iter().map(|x| { x.to_string() }).collect();
        self.add_with_children(name_, children_);
    }

    /// Runs a depth-first-search on the look-up tree
    pub fn dfs(&self, start: String) -> Result<Vec<String>, &str> {
        let mut stack: Vec<usize> = Vec::new();
        self.traverse(start, &mut stack)
    }

    /// Runs a breadth-first-search on the look-up tree
    pub fn bfs(&self, start: String) -> Result<Vec<String>, &str> {
        let mut queue: VecDeque<usize> = VecDeque::new();
        self.traverse(start, &mut queue)
    }

    /// Parse a line for creating a look-up tree from file
    fn parse_line(line: &str) -> (String, Vec<String>) {
        let parts: Vec<&str> = line.split("->").collect();
        let children: Vec<&str> = parts[1].split(",").collect();
        (parts[0].to_string(), children.iter().map(|x| x.to_string()).collect())
    }

    /// Add a node with its children to the look-up tree
    pub fn add_with_children(&mut self, name: String, children: Vec<String>) {
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
    pub fn add_to_parent(&mut self, child: String, parent: String) -> Result<usize, &str> {
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

    /// Walks the look-up tree with a predefined buffer
    /// 
    /// # Arguments
    /// 
    /// * `start` - A String defining the name of the start node for traversal
    /// * `buffer` - A SearchBuffer, e.g. a stack for DFS or a queue for BFS
    pub fn traverse(&self, start: String, buffer: &mut dyn SearchBuffer<usize>) -> Result<Vec<String>, &str> {
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


#[cfg(test)]
mod test {

    use super::NamedLuTree;

    fn assert_lutree_field_len(lutree: &NamedLuTree, len: usize) {
        assert_eq!(lutree.names.len(), len);
        assert_eq!(lutree.parents.len(), len);
        assert_eq!(lutree.children.len(), len);
    }

    #[test]
    fn using_box_vec() {
        let bv = vec![Box::new("a")];
        assert_eq!(bv.contains(&Box::new("a")), true);
        assert_eq!(bv.contains(&Box::new("b")), false);
    }

    #[test]
    fn test_lutree_new() {
        let gr = NamedLuTree::new();
        assert_lutree_field_len(&gr, 0);
    }

    #[test]
    fn test_add_to_parent() {
        let mut gr = NamedLuTree::new();

        // Add a node without parent
        gr.add_with_children_r("A", Vec::new());

        // Add children to root node
        let _ = gr.add_to_parent("B".to_string(), "A".to_string());
        assert_lutree_field_len(&gr, 2);
        assert_eq!(gr.children[0].len(), 1);

        // Reject existing node
        let mut res = gr.add_to_parent("A".to_string(), "B".to_string());
        assert_eq!(res, Err("Node exists already"));

        // Add node with exotic name
        res = gr.add_to_parent("Wéïrd näµëß§".to_string(), "A".to_string());
        assert_eq!(res, Ok(2));
        assert_lutree_field_len(&gr, 3);
    }

    #[test]
    fn test_add_with_children() {
        let mut gr = NamedLuTree::new();

        // Add node without parent
        gr.add_with_children_r("Root", Vec::new());
        assert_lutree_field_len(&gr, 1);

        // Add children to existing parent
        gr.add_with_children_r("Root", vec!["A"]);
        assert_lutree_field_len(&gr, 2);

        // Add more children
        let parent = "A";
        let children = vec!["B", "C", "D"];
        gr.add_with_children_r(parent, children);

        assert_lutree_field_len(&gr, 5);
        assert_eq!(gr.children[1].len(), 3);
    }

    #[test]
    fn test_parse_line() {
        let l1 = "A->B,C";
        let (parent1, children1) = NamedLuTree::parse_line(l1);
        let ref_children = vec!["B".to_string(), "C".to_string()];
        assert_eq!(parent1, "A".to_string());
        assert_eq!(children1, ref_children);
    }
}
