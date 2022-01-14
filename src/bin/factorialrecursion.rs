fn factorial(arg: i128) -> i128 {
    if arg == 1 {
        return arg;
    }
    return arg * factorial(arg - 1);
}

fn main() {
    let num: i128 = 4;
    let result = factorial(num);
    println!("{result}");
}
