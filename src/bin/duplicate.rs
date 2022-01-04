// check if duplicate exist in array of number (by frquency counter)

use std::collections::HashMap;

fn are_there_duplicates(arr: Vec<i32>) -> bool {
    let mut obj: HashMap<i32, usize> = HashMap::new();
    for i in 0..arr.len() {
        obj.entry(arr[i]).and_modify(|v| *v += 1).or_insert(1);
    }
    for (key, _value) in &obj {
        if obj.get(&key).unwrap() > &1 {
            return true;
        }
    }
    return false;
}

fn main() {
    let newvec = vec![1, 2, 3];
    let result = are_there_duplicates(newvec);
    println!("{:?}", result);
}
