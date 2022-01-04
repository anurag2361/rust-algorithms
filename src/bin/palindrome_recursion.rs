fn is_palindrome(string: &str) -> bool {
    if string.len() < 2 {
        return true;
    }
    if string.chars().nth(0) == string.chars().nth(string.len() - 1) {
        return is_palindrome(&string[1..string.len() - 1]);
    }
    return false;
}

fn main() {
    let s = String::from("kayak");
    println!("{}", is_palindrome(&s));
}
