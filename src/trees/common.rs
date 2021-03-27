use std::collections::VecDeque;

/// Trait containing node buffers used during search
/// 
/// Abstracts a stack and a queue for look-up tree traversal
pub trait SearchBuffer<T> {
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

#[derive(Debug,Clone,PartialEq)]
pub struct NodeData {
    pub value: f32
}