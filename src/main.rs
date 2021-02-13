
use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    names: Vec<Box<String>>,
    parents: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
    name2idx: HashMap<String, usize>,
}

impl Graph {

    fn new() -> Graph {
        Graph { names: Vec::new(), parents: Vec::new(), children: Vec::new(), name2idx: HashMap::new() }
    }

    fn add_with_parent_r(&mut self, name: String, parent: Option<String>) -> Result<usize, &str> {
        let node_idx = self.parents.len();

        match parent {
            Some(p) => {
                match self.name2idx.get(&p) {
                    Some(&p_idx) => {
                        self.parents.push(Some(p_idx));
                        // Update children of parent
                        self.children[p_idx].push(node_idx);
                    },
                    None => return Err("Unknown key"),
                }
            },
            None => self.parents.push(None),
        }

        let name_ = name.clone();

        // Add new node with parent
        self.names.push(Box::new(name));
        self.children.push(Vec::new());

        self.name2idx.insert(name_, node_idx);
        Ok(node_idx)
    }

    fn add_with_parent(&mut self, name: String, parent: Option<String>) {
        match self.add_with_parent_r(name, parent) {
            Ok(node_idx) => println!("Added node {}", node_idx),
            Err(msg) => println!("Fail: {}", msg),
        }
    }

    fn add_with_children(&mut self, name: String, children: Vec<String>) {
        let node_idx = self.parents.len();

        // Check if parent is already there
        match self.names.contains(&Box::new(name.clone())) {
            true => {
                // Parent node exists already - check for children
                // Add children
                for child in children.iter() {
                    self.add_with_parent((*child).clone(), Some(name.clone()));
                }
            },
            false => {
                // Add parent node
                self.names.push(Box::new(name.clone()));
                self.parents.push(None);
                self.name2idx.insert(name.clone(), node_idx);
                self.children.push(Vec::new());

                // Add children
                for child in children.iter() {
                    self.add_with_parent((*child).clone(), Some(name.clone()));
                }
            },
        }
    }
}


fn main() {
    println!("A graph of nodes in Rust!");

    let mut gr = Graph::new();

    gr.add_with_parent("A".to_string(), None);
    gr.add_with_parent("B".to_string(), Some("A".to_string()));
    gr.add_with_parent("C".to_string(), Some("A".to_string()));

    println!("{:?}", gr);
}


#[cfg(test)]
mod test {
    use super::Graph;

    fn graph_fields_eq_len(graph: &Graph, len: usize) {
        assert_eq!(graph.names.len(), len);
        assert_eq!(graph.parents.len(), len);
        assert_eq!(graph.children.len(), len);
    }

    #[test]
    fn using_box_vec() {
        let bv = vec![Box::new("a".to_string())];
        assert_eq!(bv.contains(&Box::new("a".to_string())), true);
        assert_eq!(bv.contains(&Box::new("b".to_string())), false);
    }

    #[test]
    fn test_add_with_children() {
        let mut gr = Graph::new();
        let parent = "A".to_string();
        let children = vec!["B".to_string(), "C".to_string(), "D".to_string()];
        gr.add_with_children(parent, children);

        assert_eq!(gr.names.len(), 4);
        assert_eq!(gr.children[0].len(), 3);
    }

    #[test]
    fn initialize_graph() {
        let gr = Graph::new();
        graph_fields_eq_len(&gr, 0);
    }

    #[test]
    fn test_add_with_parent_r() {
        let mut gr = Graph::new();

        // Add a node without parent
        let mut res = gr.add_with_parent_r("A".to_string(), None);
        assert_eq!(res, Ok(0));

        // Add a node with an existing parent
        res = gr.add_with_parent_r("B".to_string(), Some("A".to_string()));
        assert_eq!(res, Ok(1));

        // Try to add a node with a non-existant parent
        graph_fields_eq_len(&gr, 2);
        res = gr.add_with_parent_r("Test".to_string(), Some("NonExistant".to_string()));
        assert_eq!(res, Err("Unknown key"));
        graph_fields_eq_len(&gr, 2);

        // Try adding a node with spaces and unicode name
        res = gr.add_with_parent_r("Wéird ßpäßes".to_string(), Some("B".to_string()));
        assert_eq!(res, Ok(2));
        
        // Reference the weird node
        res = gr.add_with_parent_r("C".to_string(), Some("Wéird ßpäßes".to_string()));
        assert_eq!(res, Ok(3));
        graph_fields_eq_len(&gr, 4);
    }

    #[test]
    fn test_add_with_parent() {
        // To see the output, run with `-- --nocapture`
        let mut gr = Graph::new();

        // Add a node without parent
        gr.add_with_parent("A".to_string(), None);

        // Add a node with a parent
        gr.add_with_parent("B".to_string(), Some("A".to_string()));

        // Add a node without a parent
        gr.add_with_parent("C".to_string(), Some("NonExistant".to_string()));

    }
}

// TODO
// Reading Graph from File
// Upstream
// Downstream
// Searches
// Reject existing node
