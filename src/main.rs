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

fn map_book<'a >(letter: Option<char>, dir: bool,
            mut book: &'a mut HashMap<char, Vec<bool>>) -> &'a HashMap<char, Vec<bool>> {

    book = match letter {
        Some(letter) => {
            book.insert(letter, vec![dir]);
            book
        },
        None => {
            for (_, value) in book.iter_mut() {
                value.push(dir);
            }
            book
        }
    };

    book
}

fn main() {
    let a = "Tree";

    let chars: Vec<_> = a.chars().collect();
    let mut freq = HashMap::new();

    for letter in chars {
        let mut node_book = HashMap::new();
        node_book.insert(letter, vec![false]);
        let node = HuffNode::new(1, Some(letter), Some(node_book));
        freq.entry(letter)
            .and_modify(|e: &mut HuffNode| {
                e.count += 1;
                e.book.insert(letter, vec![false]);
            })
            .or_insert(node);
    }

    let mut huff_nodes: Vec<HuffNode> = freq.into_iter().map(|e| e.1).collect();

    while huff_nodes.len() > 1 {
        huff_nodes.sort_by(|a, b| (&(b.count)).cmp(&(a.count)));
        let mut n1 = huff_nodes.pop().unwrap();
        let mut n2 = huff_nodes.pop().unwrap();

        let n1_ref = &mut n1;
        let n2_ref = &mut n2;

        let mapped_book_left = map_book(n1_ref.letter, true, &mut n1_ref.book);
        let mapped_book_right = map_book(n2_ref.letter, false, &mut n2_ref.book);

        let book: HashMap<char, Vec<bool>> = mapped_book_left
            .to_owned()
            .into_iter()
            .chain(mapped_book_right.to_owned())
            .collect();

        let n_new = HuffNode::new(n1.count + n2.count, None, Some(book))
            .left(n1)
            .right(n2);

        huff_nodes.push(n_new);
    }

    let tree: HuffNode = huff_nodes.pop().unwrap();

    println!("CodeBook for Tree: {:#?}", tree);
}
