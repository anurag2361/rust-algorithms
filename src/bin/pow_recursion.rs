// raising a base to the power of its exponent by recursion

fn power(base: i32, exponent: i32) -> i32 {
    if exponent == 0 {
        return 1;
    }
    return base * power(base, exponent - 1);
}

fn main() {
    println!("{}", power(2, 3));
}
