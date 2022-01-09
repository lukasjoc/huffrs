use std::collections::*;

#[derive(Debug, Default)]
struct HuffNode {
    count: i32,
    letter: Option<char>,
    left:  Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

impl HuffNode {
    fn new(count: i32, letter: Option<char>) -> Self {
        Self {
            count,
            letter,
            ..Default::default()
        }
    }
    fn left(mut self, node: HuffNode) -> Self {
        self.left = Some(Box::new(node));
        self
    }
    fn right(mut self, node: HuffNode) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

fn main() {
    let a = "Huffman";

    let chars: Vec<_> = a.chars().collect();
    let mut freq = HashMap::new();
    for letter in chars {
        freq.entry(letter)
        .and_modify(|e: &mut HuffNode| e.count+=1 )
        .or_insert(HuffNode::new(1, Some(letter)));
    }

    let mut huff_nodes: Vec<HuffNode> = freq.into_iter().map(|e| e.1).collect();

    while huff_nodes.len() > 1 {
        huff_nodes.sort_by(|a, b| (&(b.count)).cmp(&(a.count)));
 
        let n1 = huff_nodes.pop().unwrap();
        let n2 = huff_nodes.pop().unwrap();

        let n_new = HuffNode::new(n1.count+n2.count, None)
            .left(n1)
            .right(n2);

        huff_nodes.push(n_new);
    }

    println!("{:#?}", huff_nodes);
}