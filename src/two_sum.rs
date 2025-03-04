/*
https://leetcode.com/problems/two-sum/description/

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.


>>>>> Example 1:

Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].

>>>>> Example 2:

Input: nums = [3,2,4], target = 6
Output: [1,2]


>>>>>> Example 3:

Input: nums = [3,3], target = 6
Output: [0,1]

*/

fn solution1(nums: &Vec<i32>, target: &i32) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n {
        for j in 0..n {
            if i != j && nums[i] + nums[j] == *target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}


/*
-> Need to implement with Hash table
*/
use std::collections::HashMap;

fn solution2(nums:Vec<i32>, target: i32) -> Ven<i32>{
    // need to learn how ot use hashmap
}




pub fn two_sum() {
    let input1 = vec![2, 7, 11, 15];
    let target: i32 = 9;
    println!(
        "This is Solution 1 -> Input array {:?}, target is {:?}, result is {:?}",
        &input1,
        &target,
        solution1(&input1, &target)
    );

    println!(
        "This is Solution 1 -> Input array {:?}, target is {:?}, result is {:?}",
        &input1,
        &target,
        solution2(&input1, &target)
    );
}

