
use std::collections::HashMap;

#[derive(Debug)]
struct Graph {
    names: Vec<Box<String>>,
    name2idx: HashMap<String, usize>,
    parents: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
}

impl Graph {

    fn new() -> Graph {
        Graph { names: Vec::new(), parents: Vec::new(), children: Vec::new(), name2idx: HashMap::new() }
    }

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
}


fn main() {
    println!("A graph of nodes in Rust!");

    // let mut gr = Graph::new();
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
        let bv = vec![Box::new("a".to_string())];
        assert_eq!(bv.contains(&Box::new("a".to_string())), true);
        assert_eq!(bv.contains(&Box::new("b".to_string())), false);
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
        gr.add_with_children("A".to_string(), Vec::new());

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
        gr.add_with_children("Root".to_string(), Vec::new());
        assert_graph_field_len(&gr, 1);

        // Add children to existing parent
        gr.add_with_children("Root".to_string(), vec!["A".to_string()]);
        assert_graph_field_len(&gr, 2);

        // Add more children
        let parent = "A".to_string();
        let children = vec!["B".to_string(), "C".to_string(), "D".to_string()];
        gr.add_with_children(parent, children);

        assert_graph_field_len(&gr, 5);
        assert_eq!(gr.children[1].len(), 3);
    }
}

// TODO
// Reading Graph from File
// Upstream
// Downstream
// Searches
