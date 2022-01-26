fn selection_sort(mut arr: Vec<i32>) -> Vec<i32> {
    for i in 0..arr.len() {
        let mut smallest = i;
        for j in i..arr.len() {
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        if i != smallest {
            arr.swap(i, smallest);
        }
    }
    arr
}

fn main() {
    println!("{:?}", selection_sort([6, 5, 4, 3, 2, 1].to_vec()));
}
