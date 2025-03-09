/*
268. Missing Number
Easy
Topics
Companies
Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.


Example 1:

    Input: nums = [3,0,1]

    Output: 2

    Explanation:

        n = 3 since there are 3 numbers, so all numbers are in the range [0,3]. 2 is the missing number in the range since it does not appear in nums.

Example 2:

    Input: nums = [0,1]

    Output: 2

    Explanation:

        n = 2 since there are 2 numbers, so all numbers are in the range [0,2]. 2 is the missing number in the range since it does not appear in nums.

Example 3:

    Input: nums = [9,6,4,2,3,5,7,0,1]

    Output: 8

    Explanation:

        n = 9 since there are 9 numbers, so all numbers are in the range [0,9]. 8 is the missing number in the range since it does not appear in nums.

*/

fn solution_1(nums:&Vec<i32>) -> i32{
    let n = nums.len();

    let sum_expected = n * (n + 1) / 2;
    let sum_actual: i32 = nums.iter().sum(); 
    
    sum_expected as i32 - sum_actual as i32

}

pub fn main(){
    let input_1 = vec![3, 0, 1];
    let input_2 = vec![0, 1];
    let input_3 = vec![9,6,4,2,3,5,7,0,1];


    println!("This is Solution 1 : The input: {:?} for output {}",&input_1, solution_1(&input_1));
    println!("This is Solution 1 : The input: {:?} for output {}",&input_2, solution_1(&input_2));
    println!("This is Solution 1 : The input: {:?} for output {}",&input_3, solution_1(&input_3));

}
