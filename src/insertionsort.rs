fn insertion_sort<T: Ord>(arr: &mut [T]) -> &mut [T] {
    let n = arr.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
    return arr;
}

fn main() {
    let mut newvec = vec![4, 2, 3, 1];
    let result = insertion_sort(&mut newvec);
    println!("{:?}", result);
}
