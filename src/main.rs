use std::io;

fn binary_search(nums: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = nums.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        match nums[mid].cmp(&target) {
            std::cmp::Ordering::Equal => return Some(mid),
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
        }
    }

    None
}

fn main() {
    let mut input = String::new();

    // Read the input array from the user
    println!("Enter a comma-separated list of integers:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let nums: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    // Read the target value from the user
    println!("Enter the target value:");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let target: i32 = input.trim().parse().unwrap();

    // Perform the binary search
    if let Some(index) = binary_search(&nums, target) {
        println!("Found {} at index {}", target, index);
    } else {
        println!("{} not found", target);
    }
}
