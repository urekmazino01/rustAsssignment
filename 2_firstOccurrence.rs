fn find_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            // Found the target element, but it might not be the first occurrence
            // Check if the previous element is also equal to the target
            if mid == 0 || arr[mid - 1] != target {
                return Some(mid); // This is the first occurrence
            } else {
                right = mid; // Continue searching in the left half for the first occurrence
            }
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None // Target not found in the array
}

fn main() {
    let sorted_array = vec![1, 2, 2, 3, 4, 4, 4, 5, 6, 7];
    let target = 7;

    match find_first_occurrence(&sorted_array, target) {
        Some(index) => println!("First occurrence of {} is at index {}.", target, index),
        None => println!("{} not found in the array.", target),
    }
}
