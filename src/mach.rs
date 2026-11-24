use std::collections::HashMap;

struct Node {
    accept: bool,
    transitions: HashMap<char, i32> 
}

impl Node {
    fn new(accept: bool, transitions: HashMap<char, i32> ) -> Self{
       Self { accept, transitions} 
    }
}

pub fn generate_machine() -> Vec<Node>{
    let mut nodes: Vec<Node> = Vec::new();
    let node: Node = Node{
        accept: true,
        transitions: HashMap::new(),
    };
    nodes.push(node);
    return nodes
}
