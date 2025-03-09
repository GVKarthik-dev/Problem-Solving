/*
217. Contains Duplicate

Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.


Example 1:

Input: 
    nums = [1,2,3,1]

Output: true

Explanation:

    The element 1 occurs at the indices 0 and 3.

Example 2:

Input: 
    nums = [1,2,3,4]

Output: false

Explanation:

    All elements are distinct.

Example 3:

    Input: nums = [1,1,1,3,3,4,3,2,4,2]

Output: true

Constraints:

    1 <= nums.length <= 105
    -109 <= nums[i] <= 109

*/

use std::{collections::HashMap};

fn solution_1(nums: &Vec<i32>) -> bool{
    let mut val_count: HashMap<i32, i32> = HashMap::new();

    for &num in nums{
        *val_count.entry(num).or_insert(0) += 1;
    }
    // println!("{:?}", &mut val_count);
    val_count.values().any(|&count| count >1)
}



pub fn main(){
    let input1 = vec![1,2,3,4];
    let input2 = vec![1,2,3,1];
    let input3 = vec![1,1,1,3,3,4,3,2,4,2];

    println!("This is Solution 1 : The input: {:?} for output {}",&input1, solution_1(&input1));
    println!("This is Solution 1 : The input: {:?} for output {}",&input2, solution_1(&input2));
    println!("This is Solution 1 : The input: {:?} for output {}",&input3, solution_1(&input3));


    // println!("This is Solution 2 : The input: {:?} for output {}",&input1, solution_2(&input1));
    // println!("This is Solution 2 : The input: {:?} for output {}",&input2, solution_2(&input2));
}