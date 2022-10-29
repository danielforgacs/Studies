enum NodeLink {
    Node(Box<Node>),
    Text(String),
}

struct Node {
    left: NodeLink,
    right: Option<NodeLink>
}

fn main() {
    let node0 = Node {
        left: NodeLink::Text("A".to_string()),
        right: Option::Some(NodeLink::Node(Box::new(
            Node {
                left: NodeLink::Node(Box::new(
                    Node {
                        left: NodeLink::Text("B".to_string()),
                        right: Some(NodeLink::Text("C".to_string())),
                    }
                )),
                right: Option::None,
            }
        )))
    };
    node_visitor(node0);
}

fn node_visitor(node: Node) {
    match node.left {
        NodeLink::Text(text) => println!("{}", text),
        _ => (),
    }
    match node.right {
        Option::Some(NodeLink::Text(text)) => println!("{}", text),
        _ => (),
    }
}
