fn kth_smallest_element(nums: &mut [i32], k: usize) -> Option<i32> {
    if k > 0 && k <= nums.len() {
        nums.sort(); // Sort the array in ascending order
        Some(nums[k - 1]) // Return the kth smallest element
    } else {
        None // Return None if k is out of range
    }
}

fn main() {
    let mut array = vec![12, 3, 1, 5, 6, 2, 10];
    let k = 3;

    match kth_smallest_element(&mut array, k) {
        Some(smallest) => println!("The {}th smallest element is: {}", k, smallest),
        None => println!("Invalid value of k."),
    }
}
