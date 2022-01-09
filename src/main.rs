use std::collections::*;

#[derive(Debug, Default)]
struct HuffNode {
    count: i32,
    book: HashMap<char, Vec<bool>>,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

impl HuffNode {
    fn new(count: i32, book: Option<HashMap<char, Vec<bool>>>) -> Self {
        Self {
            count,
            book: match book {
                Some(book) => book,
                None => HashMap::new()
            },
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
    let a = "Tree";

    let chars: Vec<_> = a.chars().collect();
    let mut freq = HashMap::new();

    for letter in chars {
        freq.entry(letter)
            .and_modify(|e: &mut HuffNode| {
                e.count += 1;
                e.book.insert(letter, vec![false]);
            })
            .or_insert(HuffNode::new(1, None));
    }

    let mut huff_nodes: Vec<HuffNode> = freq.into_iter().map(|e| e.1).collect();
    // println!("{:#?}", huff_nodes);

    while huff_nodes.len() > 1 {
        huff_nodes.sort_by(|a, b| (&(b.count)).cmp(&(a.count)));
        let n1 = huff_nodes.pop().unwrap();
        let n2 = huff_nodes.pop().unwrap();

        let n1_ref = &n1;
        let n2_ref = &n2;

        let book34: HashMap<char, Vec<bool>> = n1_ref
            .book
            .to_owned()
            .into_iter()
            .chain(n2_ref.book.to_owned())
            .collect();

        println!("Books {:?}", book34);

        let n_new = HuffNode::new(n1.count + n2.count, Some(book34)).left(n1).right(n2);

        huff_nodes.push(n_new);
    }

    let tree: HuffNode = huff_nodes.pop().unwrap();

    // println!("{:#?}", tree);
}
