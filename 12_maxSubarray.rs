fn max_subarray_sum(nums: &[i32]) -> i32 {
    let mut max_sum = nums[0];
    let mut current_sum = nums[0];

    for &num in nums.iter().skip(1) {
        current_sum = current_sum.max(num);
        max_sum = max_sum.max(current_sum);
    }

    max_sum
}

fn main() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]; // Replace with your array of numbers

    let max_sum = max_subarray_sum(&nums);

    println!("Maximum subarray sum: {}", max_sum);
}
