fn find_median(nums: &[i32]) -> f64 {
    let n = nums.len();
    if n % 2 == 0 {
        // If the number of elements is even, return the average of the middle two elements
        let mid1 = nums[n / 2 - 1] as f64;
        let mid2 = nums[n / 2] as f64;
        (mid1 + mid2) / 2.0
    } else {
        // If the number of elements is odd, return the middle element
        nums[n / 2] as f64
    }
}

fn main() {
    let sorted_array = vec![1, 2, 3, 4, 9];

    let median = find_median(&sorted_array);
    println!("The median is: {}", median);
}
