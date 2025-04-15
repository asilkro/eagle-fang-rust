use std::collections::HashSet;

fn main() {
    let input_vec = vec![1, 3, 4, 4, 1, 5, 2, 3, 2, 4, 1, 5];
  // Expected output [1, 3, 4, 5, 2]
    let output = remove_duplicates(input_vec);
    println!("{:?}", output);
}

/*
    Write a function that:
    - Takes a Vec<i32> as input
    - Returns a new Vec<i32> with all duplicates removed
    - Maintains the original order of first appearances
*/

fn remove_duplicates(nums: Vec<i32>) -> Vec<i32> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for num in nums {
        if seen.insert(num) {
            result.push(num);
        }
    }

    result

}
