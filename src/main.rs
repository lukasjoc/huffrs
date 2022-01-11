use std::collections::*;
use std::io::{self, BufRead};

type HuffBook = BTreeMap<char, String>;

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

fn map_book<'a>(
    letter: Option<char>,
    dir: String,
    mut book: &'a mut HuffBook,
) -> &'a HuffBook {
    book = match letter {
        Some(letter) => {
            book.entry(letter).or_insert(dir);
            book
        }

        None => {
            for (_, value) in book.iter_mut() {
                *value = String::from(format!("{}{}", value, dir));
            }
            book
        }
    };

    book
}

#[derive(Debug)]
struct HuffTree {
    freq: FreqMap,
    head: Option<HuffNode>,
}

impl HuffTree {
    fn new(stdin: String) -> Self {
        let chars: Vec<_> = stdin.chars().collect();
        let mut freq = BTreeMap::new();

        for letter in &chars {
            let node = HuffNode::new(Some(1), Some(*letter), None);
            freq.entry(*letter)
                .and_modify(|e: &mut HuffNode| e.count = Some(*&mut e.count.unwrap() + 1))
                .or_insert(node);
        }

        Self { freq, head: None }
    }

    fn build_tree(mut self) -> Option<HuffNode> {
        let mut nodes: Vec<HuffNode> = self.freq.into_iter().map(|e| e.1).collect();

        nodes.sort_by(|a, b| b.count.cmp(&a.count));

        while nodes.len() > 1 {
            let mut lnode = nodes.pop().unwrap();
            let mut rnode = nodes.pop().unwrap();

            let lnode_ref = &mut lnode;
            let rnode_ref = &mut rnode;

            let lbook = map_book(lnode_ref.letter, String::from("1"), &mut lnode_ref.book);

            let rbook = map_book(rnode_ref.letter, String::from("0"), &mut rnode_ref.book);

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
            *v = v.chars().rev().collect();
        }

        self.head = Some(head_node);
        self.head
    }
}

fn encode<'a>(stdin: &str, enc: &'a mut String, book: &HuffBook) -> &'a str {
    let stdin_chars: Vec<char> = stdin.chars().collect();
    for l in stdin_chars {
        enc.push_str(&book[&l].to_string());
    }

    enc.as_str()
}

fn decode<'a>(stdin: &str, dec: &'a mut String, book: &HuffBook) -> &'a str {
    let mut codes: Vec<_> = book.into_iter().collect();
    codes.sort_by(|a, b| b.1.chars().count().cmp(&a.1.chars().count()));

    let mut index = 0;

    while index < stdin.chars().count() {
        // check for each code if it matches fully with its respective length
        for (k, v) in &codes {
            let v_char_count = v.chars().count();
            let v_matches = match stdin.get(index..index + v_char_count) {
                Some(sub) => sub == v.as_str(),
                None => false,
            };

            // if the substring code is the actual code then we matched successfully
            if v_matches {
                dec.push_str(&k.to_string());
                index += v_char_count - 1;

                // At this point, we know that, there is no other match possible
                break;
            }
        }
        // the normal case when the current index is not matching to any
        // item in the book. (This should not happend with a valid bitbook)
        // and a valid encoded string.
        //
        // The Encoded String *MUST* be encoded with the bitbook given to this
        // function. Otherwise it might(probably will) not work.
        index += 1;
    }

    dec.as_str()
}

fn main() -> io::Result<()> {
    let mut stdin_text = String::new();

    let mut line = String::new();
    while io::stdin().lock().read_line(&mut line).unwrap() > 0 {
        stdin_text.push_str(&line);
        line.clear();
    }

    let huff = HuffTree::new(stdin_text.clone());

    let tree = match huff.build_tree() {
        Some(tree) => tree,
        None => HuffNode::new(None, None, None),
    };

    let mut encoded_string = String::new();

    let encoded_string = encode(&stdin_text, &mut encoded_string, &tree.book);

    //println!("Stdin(Tree): {:#?}", tree);
    println!("Stdin(Decode): {}", encoded_string);

    // let a1 = String::from("Tree\n");
    // let huff1 = HuffTree::new(a1.clone());
    //
    // let tree1 = match huff1.build_tree() {
    //     Some(tree1) => tree1,
    //     None => HuffNode::new(None, None, None),
    // };
    //
    // let mut encoded_string1 = String::new();
    //
    // let encoded_string1 = encode(&a1, &mut encoded_string1, &tree1.book);
    //
    // println!("Static(Tree): {:#?}", tree1);
    // println!("Static(Decode): {}", encoded_string1);

    Ok(())
}
