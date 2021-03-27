// Copyright 2021 Simon B. Gasse



pub struct LuTree<T> {
    parents: Vec<Option<usize>>,
    children: Vec<Vec<usize>>,
    data: Vec<T>,
}

impl<T> LuTree<T> {
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
}