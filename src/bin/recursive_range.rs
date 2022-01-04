// function to take a number and add up all numbers from 0 to the passed number
// e.g. recursiveRange(6) will give 21

fn recursive_range(num: i32) -> i32 {
    let total = 1;
    if num == 1 {
        return total;
    }
    return (num + recursive_range(num - 1));
}

fn main() {
    let result = recursive_range(4);
    println!("{}", result);
}
