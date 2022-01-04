// function to return the product of an array
// [1,2,3] returns 6

fn array_product(arr: Vec<usize>) -> usize {
    if arr.is_empty() {
        return 1;
    }
    return arr[0] * array_product((arr[1..]).to_vec());
}

fn main() {
    let vec = vec![1, 2, 3];
    let result = array_product(vec);
    println!("{:?}", result);
}
