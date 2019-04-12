use std::collections::HashMap;

fn main() {
    let mut graph: HashMap<char, Vec<char>> = HashMap::new();

    graph.insert('a', vec!['b', 'c']);
    graph.insert('b', vec!['a']);
    graph.insert('c', vec!['a']);
    graph.insert('d', vec![]);
}

fn add_edge( graph: HashMap<char, Vec<char>>, node1: char, node2: char ) {
}
