use std::convert::TryInto;

fn binary_search(mut arr: Vec<i16>, value: i16) -> i16 {
    arr.sort();
    let mut first_index = 0;
    let mut last_index = arr.len() - 1;
    let mut middle_index = ((first_index + last_index) as f32 / 2.0).floor() as usize;
    while arr[middle_index] != value && first_index < last_index {
        if arr[middle_index] < value {
            first_index = middle_index + 1;
        } else if arr[middle_index] > value {
            last_index = middle_index - 1;
        }
        middle_index = ((first_index + last_index) as f32 / 2.0).floor() as usize;
    }
    if arr[middle_index] == value {
        return middle_index.try_into().unwrap();
    } else {
        return -1;
    }
}

fn main() {
    let array: Vec<i16> = vec![10, 2, 3, 4, 5, 6, 7, 8, 9, 1];
    let value_to_search: i16 = 10;
    let result = binary_search(array, value_to_search);
    println!("Result: {result}");
}
