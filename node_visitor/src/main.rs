enum Node {
    NodeL(Box<Node>),
    NodeR(Option<Box<Node>>),
    Text(String),
    End,
}

fn main() {
    let node0 = Node::End;
    node_visitor(node0);
}

fn node_visitor(node: Node) {
    match node {
        Node::NodeL(node) => (),
        Node::NodeR(Option::Some(node)) => (),
        Node::NodeR(Option::None) => (),
        Node::Text(text) => (),
        Node::End => (),
    }
}
