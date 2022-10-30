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
    println!("---");
    let node_h = Box::new(Node { left: NodeLink::Text("H".to_string()), right: Option::None });
    let node_e = Box::new(Node { left: NodeLink::Text("E".to_string()), right: Option::None });
    let node_l = Box::new(Node { left: NodeLink::Text("L".to_string()), right: Option::None });
    let node_lo = Box::new(Node { left: NodeLink::Text("L".to_string()), right: Option::Some(NodeLink::Text("O".to_string())) });
    let root = Box::new(Node {
        left: NodeLink::Node(node_l),
        right: Some(NodeLink::Node(node_lo)),
    });
    let root = Box::new(Node {
        left: NodeLink::Node(node_e),
        right: Some(NodeLink::Node(root)),
    });
    let root = Box::new(Node {
        left: NodeLink::Node(node_h),
        right: Some(NodeLink::Node(root)),
    });
    node_visitor(*root);
}

fn node_visitor(node: Node) {
    match node.left {
        NodeLink::Node(node) => node_visitor(*node),
        NodeLink::Text(text) => println!("{}", text),
    }
    match node.right {
        Option::Some(NodeLink::Node(node)) => node_visitor(*node),
        Option::Some(NodeLink::Text(text)) => println!("{}", text),
        Option::None => {},
    }
}
