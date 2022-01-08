use std::collections::*;

// TODO:
// take sorted list of nodes by freq for each item

// take last 2 nodes and combine to new nodes with left and right Branch

// update sorted list and remove used nodes

// do again until freq of last 2 nodes match to 1

// then done and tree should be build and contained in the last node

#[derive(Debug)]
struct HuffNode {
    count: i32,
    letter: char,
    // left: ...
    // right: ...
}

impl HuffNode {
    fn new(letter: char, count: i32) -> Self {
        Self {
            count,
            letter,
        }
    }
}

#[derive(Debug)]
struct HuffTree {
    // this is the final node in generation
    head: Option<HuffNode>,
}

#[derive(Default)]
struct HuffMap {
    map: BTreeMap<char, HuffNode>,
}

impl HuffMap {
    fn new() -> Self{
        Self {
           map: BTreeMap::new(),
        }
    }
    fn get_last_2(mut self) -> Self {
        self
    }
    fn sort_by_count(mut self) -> Self {
        self
    }
}


impl HuffTree {
    fn new() -> Self {
        Self { head: None }
    }
    fn get_tree(mut self, a: &str) -> Option<HuffNode> {
        let head = None;
        let mut map = HuffMap::new();
        
        let chars: Vec<_> = a.chars().collect();
        for letter in chars {
            map.map.entry(letter)
            .and_modify(|e: &mut HuffNode| e.count+=1 )
            .or_insert(HuffNode::new(letter, 1));
        }

        // initial sort by count
        map.sort_by_count();

        // loop while 2 nodes left
        while map.map.len() > 2 {
            
            // determine the last 2 nodes
            let last_2 = map.get_last_2();

            // build a new node habving the 2 last nodes as branches
            // and the count as the accum of the 2 nodes
            let new_node = HuffNode::new(left: last_2.0,
                                         right, right: last_2.1,
                                         count: last_2.count + last_1.count);
            
            
            // remove the last 2 nodes from the map of available Nodes
            map.remove_last_2();

            // insert the new node
            map.insert(new_node);

            // sort by count again because of insertion
            map.sort_by_count();
        }

        // combine the last 2 nodes to the final huffman node
        // and save as head node
        head = self.map_to_tree();

        println!("{:#?}", map.map);

        self.head = head;
        self.head
    }
}

fn main() {
    let a = "Tree";

    let tree = HuffTree::new().get_tree(a);

    println!("{:#?}", tree);
}