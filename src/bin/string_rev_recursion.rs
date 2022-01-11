fn reverse(string: &str) -> String {
    if string.len() <= 1 {
        return string.to_string();
    } else {
        let first_len = string.chars().next().unwrap().len_utf8();
        return reverse(&string[first_len..]).to_string() + &string[0..first_len];
    }
}

fn main() {
    let name = "😒thermodynamics";
    println!("{}", reverse(name));
}
