use std::fmt;

#[derive(Debug)]
struct Node<T> {
    data: T,
    children:  Vec<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(data: T) -> Node<T> {
        Node {
            data,
            children: vec![]
        }
    }
    fn add_child(&mut self, new_node: &mut Node<T>) {
        self.children.push(Box::new(new_node));
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.data)?;
        for child in &self.children {
            write!(f, "`- {}\n", child.data)?;
        }
        write!(f, "")
    }
}

fn main() {
    let mut root   = Node::new(0);
    let mut child1 = Node::new(1);
    let mut child11    = Node::new(11);
    let mut child12    = Node::new(12);
    let mut child2     = Node::new(2);
    let mut child3     = Node::new(3);

    root.add_child(&mut child1);
    root.add_child(&mut child2);
    root.add_child(&mut child3);

    child1.add_child(&mut child11);
    child1.add_child(&mut child12);

    println!("{}", root);
}
