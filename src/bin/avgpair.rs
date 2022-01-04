// write a function to accept array and a value. If the average of a pair of values in the array
// result to the value, return true else false

fn avg_pair(arr: Vec<i16>, avg: f32) -> bool {
    if arr.is_empty() {
        return false;
    }
    let mut left: usize = 0;
    let mut right: usize = arr.len() - 1;
    while left < right {
        let sum: i16 = arr[left] + arr[right];
        let current_avg: f32 = sum as f32 / 2.0;
        if current_avg == avg {
            return true;
        } else if current_avg > avg {
            right -= 1;
        } else {
            left += 1;
        }
    }
    return false;
}

fn main() {
    let arr = vec![1, 2, 3];
    let avg: f32 = 2.5;
    let result: bool = avg_pair(arr, avg);
    println!("{:?}", result);
}
