/*
136. Single Number
Easy
Topics
Companies
Hint
Given a non-empty array of integers nums, every element appears twice except for one. 
    Find that single one.

You must implement a solution with a linear runtime complexity and use only constant extra space.


Example 1:

    Input: nums = [2,2,1]

    Output: 1

Example 2:

    Input: nums = [4,1,2,1,2]

    Output: 4

Example 3:

    Input: nums = [1]

    Output: 1



Constraints:

1 <= nums.length <= 3 * 104
-3 * 104 <= nums[i] <= 3 * 104
Each element in the array appears twice except for one element which appears only once.
*/

use std::collections::HashSet;

fn solution_1(nums: Vec<i32>) -> Option<i32>{
    let mut seen = HashSet::new();

    for num in nums{
        if seen.contains(&num){
            seen.remove(&num);
        }
        else{
            seen.insert(num);
        }
    }
    seen.into_iter().next()
}

fn solution_2(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    for num in nums {
        result ^= num;
    }
    result
}

pub fn main() {
    let input_1 = vec![2,2,1];
    println!(
        "This is Solution 1 -> Result for Input 1 is {:?}", 
        solution_1(input_1)
    );

    let input_1 = vec![2,2,1];
    println!(
        "This is Solution 2 -> Result for Input 1 is {:?}", 
        solution_2(input_1)
    );
}

#[cfg(test)] // This module will only be compiled and run during testing
mod tests {
    use super::*; // To bring the solution_1 function into scope

    #[test]
    fn test_case_1() {
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(solution_1(nums), Some(4));  // Expect the result to be 4
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(solution_2(nums), 4);  // Expect the result to be 4
    }

    #[test]
    fn test_case_2() {
        let nums = vec![7, 3, 3, 7, 9];
        assert_eq!(solution_1(nums), Some(9));  // The result should be 9 (appears once)
        let nums = vec![7, 3, 3, 7, 9];
        assert_eq!(solution_2(nums), 9);  // The result should be 9 (appears once)
    }

    #[test]
    fn test_case_3() {
        let nums = vec![42];
        assert_eq!(solution_1(nums), Some(42));  // Only one element, should return it
        let nums = vec![42];
        assert_eq!(solution_2(nums), 42);  // Only one element, should return it
    }

}