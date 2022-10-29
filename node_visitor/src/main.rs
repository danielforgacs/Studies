enum NodeLink {
    Node(Box<Node>),
    Text(String),
}

struct Node {
    left: NodeLink,
    right: Option<NodeLink>
}

fn main() {
    let node0 = Node { left: NodeLink::Text("A".to_string()), right: Option::None };
    node_visitor(node0);
}

fn node_visitor(node: Node) {
    match node.left {
        NodeLink::Text(text) => println!("{}", text),
        _ => (),
    }
}
