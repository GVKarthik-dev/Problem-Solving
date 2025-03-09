/*

283. Move Zeroes
Easy
Topics
Companies
Hint

    Given an integer array nums, move all 0's to the end of it while maintaining 
    the relative order of the non-zero elements.

    Note that you must do this in-place without making a copy of the array.



Example 1:

    Input: nums = [0,1,0,3,12]
    Output: [1,3,12,0,0]

Example 2:

    Input: nums = [0]
    Output: [0]


Constraints:

    1 <= nums.length <= 104
    -231 <= nums[i] <= 231 - 1


Follow up: Could you minimize the total number of operations done?


*/

fn move_zeroes(nums: &mut Vec<i32>){
    let mut n = nums.len();
    let mut last_none_zero_index = 0;

    for i in 0..n {
        if nums[i] != 0 {
            nums.swap(i, last_none_zero_index);
        }
        last_none_zero_index += 1;
    }
}


pub fn main() {
    let mut input_1 = vec![0, 1, 0, 3, 12];

    println!("The Output is {:?}", move_zeroes(&mut input_1));
}