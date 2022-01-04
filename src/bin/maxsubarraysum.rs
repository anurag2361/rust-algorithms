// sliding window pattern
// pass an array and a number. Find max sum of subarray with the length of the number
// e.g. ([1,2,3],2) should result in 5

use std::cmp::max;
use std::convert::TryInto;

fn max_sub_array_sum(arr: Vec<i32>, num: i32) -> Option<i32> {
    if arr.len() < num.try_into().unwrap() {
        return None;
    }
    let mut temp_sum: i32 = 0;
    let mut max_sum: i32 = 0;
    for i in 0..num as usize {
        max_sum = max_sum + arr[i];
    }
    temp_sum = max_sum;
    for i in num as usize..arr.len() {
        temp_sum = temp_sum - arr[i - num as usize] + arr[i];
        max_sum = max(temp_sum, max_sum);
    }
    return Some(max_sum);
}

fn main() {
    let newvec: Vec<i32> = vec![100, 200, 300, 400];
    let num: i32 = 2;
    let result = max_sub_array_sum(newvec, num);
    match result {
        Some(v) => println!("Result: {}", v),
        None => println!("Number needs to be smaller than the array size"),
    }
}
