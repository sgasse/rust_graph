#[macro_export]
macro_rules! get_from_tree {
    ($field:expr, $node_idx:expr) => {
        match $node_idx {
            n if $node_idx < $field.len() => Ok($field[n].clone()),
            _ => Err(("Access out of bounds!")),
        }
    };
}

pub struct LuTree<T> {
    parents: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
    data: Vec<T>,
}

impl<T> LuTree<T> where
    T: Clone {
    pub fn new() -> LuTree<T> {
        LuTree::<T>{
            parents: vec![],
            children: vec![],
            data: vec![],
        }

    }

    pub fn add_node(&mut self, parent: Option<usize>, data: T) -> Result<usize, ()> {
        match parent {
            Some(p) => {
                if p > self.parents.len() {
                    return Err(());
                }

                self.parents.push(Some(p));
            },
            None => self.parents.push(None),
        }

        let node_id = self.parents.len();
        self.children.push(vec![]);
        self.data.push(data);

        Ok(node_id)
    }

    pub fn set(&mut self, node: usize, data: T) -> Result<(), &str> {
        match node {
            n if node < self.data.len() => {
                self.data[n] = data;
                Ok(())
            },
            _ => Err("Access out of bounds!"),
        }
    }

    pub fn get(&self, node: usize) -> Result<T, &str> {
        get_from_tree!(self.data, node)
    }

    pub fn parent(&self, node: usize) -> Result<usize, &str> {
        let p_res = get_from_tree!(self.parents, node);
        match p_res {
            Ok(p_opt) => match p_opt {
                Some(p) => Ok(p),
                None => Err("Node has not parent"),
            },
            Err(e) => Err(e),
        }
    }

    pub fn children(&self, node: usize) -> Result<Vec<usize>, &str> {
        get_from_tree!(self.children, node)
    }
}

#[cfg(test)]
mod test {
    use super::{LuTree};

    #[test]
    fn create_lu_trees() {
        let mut i32_tree = LuTree::new();
        let _ = i32_tree.add_node(None, 34);
        let _ = i32_tree.add_node(Some(0), 12);
        let _ = i32_tree.add_node(Some(1), 55);
    }

    #[test]
    fn work_with_lu_tree() {
        // Build tree
        let mut tree = LuTree::new();
        let _ = tree.add_node(None, "0_0");
        let _ = tree.add_node(Some(0), "1_0");
        let _ = tree.add_node(Some(0), "1_1");
        let _ = tree.add_node(Some(1), "2_0");

        // Get node as parent of other node
        assert_eq!(tree.get(tree.parent(3).unwrap()).unwrap(), "1_0");

        // Get children of node
        let l1_children = tree.children(0).unwrap();
        for child in l1_children {
            assert!(child == 1 || child == 2, "Unexpected child!");
        }

        // Get data of node, update and check again
        assert_eq!(tree.get(3).unwrap(), "2_0");
        let new_data = "NewData";
        let _ = tree.set(3, new_data.clone());
        assert_eq!(tree.get(3).unwrap(), new_data);

    }
}