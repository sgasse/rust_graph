// Copyright 2021 Simon B. Gasse

use std::cell::RefCell;

#[derive(Debug,Clone,PartialEq)]
pub struct NodeData {
    value: f32
}

#[derive(Debug,PartialEq)]
pub struct Node<'graph, T> {
    parent: RefCell<Option<&'graph Node<'graph, T>>>,
    children: RefCell<Vec<&'graph Node<'graph, T>>>,
    data: RefCell<T>,
}

impl<T> Node<'_, T> {
    pub fn new<'a>(data: T) -> Node<'a, T> {
        let node = Node{
            parent: RefCell::new(None),
            children: RefCell::new(vec![]),
            data: RefCell::new(data),
        };
        node
    }
}

pub fn add_child_to_parent<'b, T>(parent: &'b Node<'b, T>, child: &'b Node<'b, T>) {
    parent.children.borrow_mut().push(child);
    child.parent.replace(Some(parent));

}

#[cfg(test)]
mod test {

    use super::Node;
    use super::NodeData;
    use super::add_child_to_parent;

    #[test]
    fn test_new_node() {
        let data = NodeData{value: 0.5};
        let n = Node::new(data.clone());
        assert_eq!(n.data.borrow().value, data.value);
    }

    #[test]
    fn test_build_graph() {
        let p = Node::new(NodeData{value: 0.1});
        let c1 = Node::new(NodeData{value: 0.3});
        let c2 = Node::new(NodeData{value: 0.4});
        add_child_to_parent(&p, &c1);
        add_child_to_parent(&p, &c2);
    }

    #[test]
    fn test_replace_data() {
        let p = Node::new(NodeData{value: 0.1});
        let c1 = Node::new(NodeData{value: 0.3});
        add_child_to_parent(&p, &c1);

        assert_eq!(p.data.borrow().value, 0.1);
        p.data.borrow_mut().value = 0.6;
        assert_eq!(p.data.borrow().value, 0.6);
    }

    #[test]
    fn test_get_children_through_parent() {
        let p = Node::new(NodeData{value: 0.1});
        let c1 = Node::new(NodeData{value: 0.3});
        add_child_to_parent(&p, &c1);

        let first_child = p.children.borrow()[0];
        assert_eq!(first_child.data.borrow().value, 0.3);

        let c1c1 = Node::new(NodeData{value:0.5});
        add_child_to_parent(first_child, &c1c1);

        let grand_child = c1.children.borrow()[0];
        assert_eq!(grand_child.data.borrow().value, 0.5);
    }
}