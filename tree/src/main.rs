#[derive(Debug)]
struct Node<T: PartialOrd> {
    data: T,
    right: Option<Box<Node<T>>>,
    left: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
    fn new(data: T) -> Node<T> {
        return Node {
            data,
            right: None,
            left: None
        }
    }
    fn insert(&mut self, data: T) {
        if data >= self.data {
            match self.right {
                Some(ref mut r) => r.insert(data),
                None => self.right = Some(Box::new(Node::new(data))),
            }
        } else {
            match self.left {
                Some(ref mut l) => l.insert(data),
                None => self.left = Some(Box::new(Node::new(data)))
            }
        }
    }
    fn post_order() {
        unimplemented!()
    }
}

fn main() {
    let mut root = Node::new("g");
    root.insert("d");
    root.insert("b");
    root.insert("a");
    root.insert("z");
    println!("{:#?}", root);
}

