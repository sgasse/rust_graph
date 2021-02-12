
#[derive(Debug)]
struct Node<'a> {
    name: String,
    parent: Option<&'a Node<'a>>,
}

fn upstream(n: Option<&Node>) {
    match n {
        Some(v) => {
            println!("{}", v.name);
            upstream(v.parent);
            },
        None => (),
    }
}

fn main() {
    println!("Hello, world!");

    let na = Node { name: String::from("A"), parent: None};
    let nb = Node { name: String::from("B"), parent: Some(&na)};
    let nc = Node { name: String::from("C"), parent: Some(&nb)};
    let nd = Node { name: String::from("D"), parent: Some(&na)};
    println!("{} is: {:?}", nc.name, nc);
    println!("{} is: {:?}", nd.name, nd);
    upstream(Some(&nc));
    
}
