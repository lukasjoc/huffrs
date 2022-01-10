use std::collections::*;

#[derive(Debug, Default)]
struct HuffNode {
    count: i32,
    letter: Option<char>,
    book: HashMap<char, Vec<bool>>,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

impl HuffNode {
    fn new(count: i32, letter: Option<char>,
            book: Option<HashMap<char, Vec<bool>>>) -> Self {
        Self {
            count,
            letter,
            book: match book {
                Some(book) => book,
                None => HashMap::new(),
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

fn map_book<'a>(letter: Option<char>, dir: bool,
        mut book: &'a mut HashMap<char, Vec<bool>> ) -> &'a HashMap<char, Vec<bool>> {
    book = match letter {
        Some(letter) => {
            book.entry(letter).or_insert(vec![dir]);
            book
        }

        None => {
            for (_, value) in book.iter_mut() {
                value.push(dir);
            }
            book
        }
    };

    book
}
fn build_tree(a: &str) -> HuffNode {
    let chars: Vec<_> = a.chars().collect();
    let mut freq = HashMap::new();

    for letter in chars {
        let node = HuffNode::new(1, Some(letter), None);
        freq.entry(letter)
            .and_modify(|e: &mut HuffNode| {
                e.count += 1;
            })
            .or_insert(node);
    }

    let mut huff_nodes: Vec<HuffNode> = freq.into_iter().map(|e| e.1).collect();
    huff_nodes.sort_by(|a, b| (&(b.count)).cmp(&(a.count)));

    while huff_nodes.len() > 1 {
        let mut left_node = huff_nodes.pop().unwrap();
        let mut right_node = huff_nodes.pop().unwrap();

        let left_node_ref = &mut left_node;
        let right_node_ref = &mut right_node;

        let left_book = map_book(left_node_ref.letter, true, &mut left_node_ref.book);
        let right_book = map_book(right_node_ref.letter, false, &mut right_node_ref.book);
        let book: HashMap<char, Vec<bool>> = left_book
            .to_owned()
            .into_iter()
            .chain(right_book.to_owned())
            .collect();

        let next_node = HuffNode::new(left_node_ref.count + right_node_ref.count, None, Some(book))
            .left(left_node)
            .right(right_node);
        huff_nodes.push(next_node);
        if huff_nodes.len() == 2 {
            huff_nodes.sort_by(|a, b| (&(b.letter)).cmp(&(a.letter)));
        } else {
            huff_nodes.sort_by(|a, b| (&(b.count)).cmp(&(a.count)));
        }
    }
    let tree = huff_nodes.pop().unwrap();
    tree
}

fn main() {
    let a = "Huffman Encoding";

    let tree = build_tree(a);

    println!("CodeBook for Tree: {:#?}", tree);
}
