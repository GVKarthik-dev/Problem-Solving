/*

977. Squares of a Sorted Array
Easy
Topics
Companies

    Given an integer array nums sorted in non-decreasing order, return an array of the squares of each number sorted in non-decreasing order.


Example 1:

    Input: nums = [-4,-1,0,3,10]
    Output: [0,1,9,16,100]
    Explanation: 
        After squaring, the array becomes [16,1,0,9,100].
        After sorting, it becomes [0,1,9,16,100].

Example 2:

    Input: nums = [-7,-3,2,3,11]
    Output: [4,9,9,49,121]


Constraints:

    1 <= nums.length <= 104
    -104 <= nums[i] <= 104
    nums is sorted in non-decreasing order.

    Follow up: Squaring each element and sorting the new array is very trivial, 
        could you find an O(n) solution using a different approach?
*/

fn solution_1(nums: Vec<i32>){
    let mut left = 0;
    let mut right = nums.len() -1;
    let mut result: Vec<i32> = Vec::new();

    while left <= right {

        if nums[left].abs() > nums[right] {
            result.push(nums[left].powi(2));
            left += 1;
        }
        else {
            result.push(nums[right].powi(2));
            right -= 1;
        }
        result.revese();
        result
    }

}


pub fn main() {
    let input_1 = vec![-4, -1, 0, 3, 10];

    println!("Solution for input 1 is {}", solution_1(input_1));
}