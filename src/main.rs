use std::collections::*;
use std::cmp::Ordering::*;

// TODO: Coabine 2 least frequent values together until created a binary tree
// TODO: Traverse the tree backwards up place 0, down place 1 to get the codebook

// TODO: move to struct ?
//struct HuffNode {
//    letter: char,
//    freq: f32,
//    bit: i8,
//}


fn main() {
    const A: &str = "Tree";

    // letter:(count,freq)
    let mut tree = BTreeMap::new();

    let freq = |len: usize, amount: i32 | -> f32 {
        1f32 / ((len as f32) / amount as f32)
    };

    let chars: Vec<char> = A.chars().collect();
    for c in chars {
        // calculate the entry
        tree.entry(c)
            .and_modify(|e| {
                let (ref mut count, ref mut fr) = e;
                *count += 1;
                *fr=freq(A.chars().count(), *count)
            })
            .or_insert((1, freq(A.chars().count(), 1i32)));
    }

    let mut tree0: Vec<(&char, &mut (i32, f32))> = tree.iter_mut().collect();

    tree0.sort_by(|a, b| b.1.1.partial_cmp(&a.1.1).unwrap_or(Equal));

    println!("{:#?}", tree0);
}

