use std::collections::BTreeMap;

// TODO: Frequence the string
// TODO: Coabine 2 least frequent values together until created a binary tree
// TODO: Traverse the tree backwards up place 0, down place 1 to get the codebook


fn main() {
    const A: &str = "A tuple is a collection of values of different types.
    Tuples are constructed using parentheses (), and each tuple itself is a
    value with type signature (T1, T2, ...), where T1, T2 are the types of its
    members. Functions can use tuples to return multiple values, as tuples can
    hold any number of values.";

    // letter:(count,freq)
    let mut tree: BTreeMap<char, (i32, f32)> = BTreeMap::new();

    let freq = |len: usize, amount: i32 | -> f32 {
        1f32 / ((len as f32) / amount as f32)
    };

    let chars: Vec<char> = A.chars().collect();
    for c in chars {

        tree.entry(c)
            .and_modify(|e| {
                let next_count = e.0 + 1;
                e.0 = next_count;
                e.1 = freq(A.chars().count(), next_count);
            })
            .or_insert((1, freq(A.chars().count(), 1i32)));
    }

    println!("{:#?}", tree);
}

