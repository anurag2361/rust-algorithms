fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut vec: Vec<i32> = vec![1, 3, 4, 5, 2];
    bubble_sort(&mut vec);
    println!("{:?}", vec);
}
