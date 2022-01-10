use std::collections::*;
use std::str;

type HuffBook = BTreeMap<char, Vec<bool>>;
type FreqMap = BTreeMap<char, HuffNode>;

#[derive(Debug, Default)]
struct HuffNode {
    count: i32,
    letter: Option<char>,
    book: HuffBook,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

impl HuffNode {
    fn new(count: i32, letter: Option<char>,
           book: Option<HuffBook>) -> Self {
        Self {
            count,
            letter,
            book: match book {
                Some(book) => book,
                None => BTreeMap::new(),
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
                mut book: &'a mut HuffBook ) -> &'a HuffBook {

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

fn build_tree(freq: FreqMap) -> HuffNode {

    let mut nodes: Vec<HuffNode> = freq.into_iter().map(|e| e.1).collect();
    nodes.sort_by(|a, b| b.count.cmp(&a.count));

    while nodes.len() > 1 {
        let mut lnode = nodes.pop().unwrap();
        let mut rnode = nodes.pop().unwrap();

        let lnode_ref = &mut lnode;
        let rnode_ref = &mut rnode;

        let lbook = map_book(lnode_ref.letter, true, &mut lnode_ref.book);
        let rbook = map_book(rnode_ref.letter, false, &mut rnode_ref.book);

        let book: HuffBook = lbook
            .to_owned()
            .into_iter()
            .chain(rbook.to_owned())
            .collect();

        let count = lnode_ref.count + rnode_ref.count;
        let node = HuffNode::new(count, None, Some(book))
            .left(lnode)
            .right(rnode);

        nodes.push(node);
        if nodes.len() == 2 {
            nodes.sort_by(|a, b| b.letter.cmp(&a.letter));
        } else {
            nodes.sort_by(|a, b| b.count.cmp(&a.count));
        }
    }

    let mut tree = nodes.pop().unwrap();
    for (_, v) in tree.book.iter_mut() {
        v.reverse();
    }

    tree
}

fn build_map(phrase: &str) -> FreqMap {
    let chars: Vec<_> = phrase.chars().collect();
    let mut freq = BTreeMap::new();

    for letter in chars {
        let node = HuffNode::new(1, Some(letter), None);
        freq.entry(letter)
            .and_modify(|e: &mut HuffNode| e.count += 1)
            .or_insert(node);
    }
    freq
}

fn main() {
    let a = "Tree"; //"ABCDEFGHIJKLMNOPQRSTUVWXYXabcdefghijklmnopqrstuvwxyz";

    // The Frequency Map
    let freq = build_map(a);

    // The Tree with BitBook and Nodes
    let tree = build_tree(freq);

    println!("CodeBook: {:#?}", tree.book);
}
