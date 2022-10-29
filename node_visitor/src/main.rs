enum Node {
    NodeL(Box<Node>),
    NodeR(Option<Box<Node>>),
    Text(String),
    End,
}

fn main() {
    let node0 = Node::Text("asdjfh".to_string());
    let node0 = Node::NodeL(
        Box::new(
            Node::Text("nested".to_string())
        )
    );
    node_visitor(node0);
}

fn node_visitor(node: Node) {
    match node {
        Node::NodeL(node) => {
            node_visitor(*node);
        },
        Node::NodeR(Option::Some(node)) => {
        },
        Node::NodeR(Option::None) => {
        },
        Node::Text(text) => {
            println!("{}", text);
        },
        Node::End => {
        },
    }
}
