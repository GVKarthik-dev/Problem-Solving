/*
448. Find All Numbers Disappeared in an Array
Easy
Topics
Companies
Hint

    Given an array nums of n integers where nums[i] is in the range [1, n], 
        return an array of all the integers in the range [1, n] that do not appear in nums.


Example 1:

    Input: nums = [4,3,2,7,8,2,3,1]
    Output: [5,6]

Example 2:

    Input: nums = [1,1]
    Output: [2]

Constraints:

    n == nums.length
    1 <= n <= 105
    1 <= nums[i] <= n

*/


fn solution_1(nums: &mut Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
        
    // Mark the visited numbers
    for &num in nums {
        let idx = num.abs() as usize - 1;  // Get the index of the number (1-based to 0-based index)
        if nums[idx] > 0 {
            nums[idx] = -nums[idx];  // Mark the index as visited by making the value negative
        }
    }

    // Collect all the indices that are still positive
    let mut missing_numbers = Vec::new();
    for (i, &num) in nums.iter().enumerate() {
        if num > 0 {
            missing_numbers.push(((i + 1) as i32).try_into().unwrap());  // Add the 1-based index to the result
        }
    }
    
    missing_numbers
}

pub fn main(){
    let mut input_1 = vec![4, 3, 2, 7,8, 2, 3, 1];
    let mut input_2 = vec![1, 1];
    let mut input_3 = vec![9,6,4,2,3,5,7,0,1]; // 8


    println!("This is Solution 1 : The input: {:?} for output {:?}",&input_1, solution_1(&mut input_1));
    println!("This is Solution 1 : The input: {:?} for output {:?}",&input_2, solution_1(&mut input_2));
    println!("This is Solution 1 : The input: {:?} for output {:?}",&input_3, solution_1(&mut input_3));

}