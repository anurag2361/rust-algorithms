fn naive_string_search(main_string: &str, search_string: &str) -> i32 {
    let mut count = 0;
    for i in 0..main_string.len() {
        for j in 0..search_string.len() {
            if search_string.chars().nth(j) != main_string.chars().nth(i + j) {
                break;
            }
            if j == search_string.len() - 1 {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let search_string = "shark";
    let result = naive_string_search("king shark", search_string);
    println!("Position of {search_string} is {result}");
}
