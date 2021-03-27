// Copyright 2021 Simon B. Gasse



pub struct LuTree<T> {
    parents: Vec<usize>,
    children: Vec<Vec<usize>>,
    data: Vec<T>,
}