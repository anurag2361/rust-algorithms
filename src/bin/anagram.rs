use std::collections::HashMap;

fn check_anagram(name1: &str, name2: &str) -> bool {
    if name1.len() != name2.len() {
        return false;
    }
    let mut obj: HashMap<char, i32> = HashMap::new();
    for character in name1.chars() {
        obj.entry(character).and_modify(|v| *v += 1).or_insert(1);
    }
    for i in 0..name2.len() {
        let key = name2.chars().nth(i).unwrap();
        if obj.contains_key(&key) {
            obj.entry(key).and_modify(|v| *v -= 1);
        } else {
            return false;
        }
    }
    return true;
}

fn main() {
    let name1 = "abc";
    let name2 = "bac";
    let result = check_anagram(name1, name2);
    println!("{result}");
}
