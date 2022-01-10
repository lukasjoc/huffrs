use std::collections::*;

type HuffBook = BTreeMap<char, Vec<bool>>;

type FreqMap = BTreeMap<char, HuffNode>;

#[derive(Debug, Default)]
struct HuffNode {
    count: Option<i32>,
    letter: Option<char>,
    book: HuffBook,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}

impl HuffNode {
    fn new(count: Option<i32>, letter: Option<char>, book: Option<HuffBook>) -> Self {
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

fn map_book<'a>(letter: Option<char>, dir: bool, mut book: &'a mut HuffBook) -> &'a HuffBook {
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

#[derive(Debug)]
struct HuffTree {
    freq: FreqMap,
    chars: Vec<char>,
    head: Option<HuffNode>,
}

impl HuffTree {
    fn new(stdin: String) -> Self {
        let chars: Vec<_> = stdin.chars().collect();
        let mut freq = BTreeMap::new();

        for letter in &chars {
            let node = HuffNode::new(Some(1), Some(*letter), None);
            freq
                .entry(*letter)
                .and_modify(|e: &mut HuffNode| e.count = Some(*&mut e.count.unwrap() + 1))
                .or_insert(node);
        }

        Self {
            freq,
            chars,
            head: None,
        }
    }

    fn build_tree(mut self) -> Option<HuffNode> {
        let mut nodes: Vec<HuffNode> = self.freq.into_iter().map(|e| e.1).collect();

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

            let count = lnode_ref.count.unwrap() + rnode_ref.count.unwrap();
            let node = HuffNode::new(Some(count), None, Some(book))
                .left(lnode)
                .right(rnode);

            nodes.push(node);
            if nodes.len() == 2 {
                nodes.sort_by(|a, b| b.letter.cmp(&a.letter));
            } else {
                nodes.sort_by(|a, b| b.count.cmp(&a.count));
            }
        }

        let mut head_node = nodes.pop().unwrap();
        for (_, v) in head_node.book.iter_mut() {
            v.reverse();
        }

        self.head = Some(head_node);
        // self
        self.head
    }

    fn encode(self) -> String {
        let mut enc = String::new();
        for l in  {
            enc = format!("{}{:?}",enc, self.head.as_ref().unwrap().book[&l]);
        }
        enc.to_string()
    }
    fn decode(self, encoded: String) -> String {
        todo!()
    }
}

fn main() {
    let a = String::from("bibbity_bobbity");

    let huff = HuffTree::new(a).build_tree();

    //let tree = match huff.build_tree() {
    //    Some(tree) => tree,
    //    None => HuffNode::new(None, None, None),
    //};
    //
    // FIX THIS DICK

    println!("{:#?}", huff.encode());
}

