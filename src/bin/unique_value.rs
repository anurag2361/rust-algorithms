use std::collections::HashSet;

fn unique_value<T: Ord + std::hash::Hash + Copy>(array: Vec<T>) -> Vec<T> {
    let mut scanned = HashSet::new();
    array
        .into_iter()
        .filter(|x| {
            if scanned.contains(x) {
                false
            } else {
                scanned.insert(*x);
                true
            }
        })
        .collect()
}

fn main() {
    let newvec1 = vec![1, 2, 3, 4, 6, 5, 4, 3, 2, 7];
    let newvec2 = vec!['a', 'b', 'c', 'a', 'c'];
    println!("{:?}", unique_value(newvec1));
    println!("{:?}", unique_value(newvec2));
}
