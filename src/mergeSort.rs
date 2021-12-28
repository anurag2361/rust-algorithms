fn merge(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;
    while i < arr1.len() && j < arr2.len() {
        if arr2[j] > arr1[i] {
            result.push(arr1[i]);
            i += 1;
        } else {
            result.push(arr2[j]);
            j += 1;
        }
    }
    while i < arr1.len() {
        result.push(arr1[i]);
        i += 1;
    }
    while j < arr2.len() {
        result.push(arr2[j]);
        j += 1;
    }
    result
}

fn merge_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let mid = ((arr.len() / 2) as f32).floor();
    let left = merge_sort((arr[0..mid as usize]).to_vec());
    let right = merge_sort((arr[mid as usize..arr.len()]).to_vec());
    return merge(left, right);
}

fn main() {
    let newvec: Vec<i32> = vec![1, 4, 3, 2, 10, 33, 22, 0];
    println!("{:?}", merge_sort(newvec));
}
