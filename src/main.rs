mod trees;

fn main() {
    println!("A tree of nodes in Rust!");

    let mut tr = trees::NamedLuTree::new();
    let _ = tr.add_with_children_r("A", vec!["B", "C", "D"]);

    println!("\n{:?}\n", &tr);

    let tr_from_file = trees::NamedLuTree::from_file("tree.txt");
    println!("\n{:?}\n", tr_from_file);

    let mut traversal = tr_from_file.dfs("A".to_string());
    println!("DFS:\n{:?}", traversal);

    traversal = tr_from_file.bfs("A".to_string());
    println!("BFS:\n{:?}", traversal);
}
