fn swap(array: &mut Vec<i32>, i: i32, j: i32) {
    let temp = array[i as usize];
    array[i as usize] = array[j as usize];
    array[j as usize] = temp;
}

fn pivot(arr: &mut Vec<i32>, start: Option<i32>) -> i32 {
    start.unwrap_or(0); // since rust doesn't support default function arguments
    let pivot_elem = arr[start.unwrap() as usize];
    let mut swap_index = start.unwrap();
    for i in start.unwrap() + 1..arr.len() as i32 {
        if pivot_elem > arr[i as usize] {
            swap_index += 1;
            swap(arr, swap_index, i);
        }
    }
    swap(arr, start.unwrap(), swap_index);
    return swap_index;
}

fn quick_sort(arr: &mut Vec<i32>, left: Option<i32>, right: Option<i32>) -> &Vec<i32> {
    left.unwrap_or(0);
    right.unwrap_or(arr.len() as i32 - 1);
    if left < right {
        let pivot_index = pivot(arr, left);
        quick_sort(arr, left, Some(pivot_index - 1));
        quick_sort(arr, Some(pivot_index + 1), right);
    }
    return arr;
}

fn main() {
    let mut newvec: Vec<i32> = vec![20, -1, 3, 7, 8, 2, -10, 43, 5, 33];
    let mut length: i32 = newvec.len() as i32;
    println!(
        "{:?}",
        quick_sort(&mut newvec, Some(0), Some(*&mut length - 1))
    );
}
