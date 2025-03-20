/*

3191. Minimum Operations to Make Binary Array Elements Equal to One I
Medium - Topics - Companies - Hint

    You are given a binary array nums.
    You can do the following operation on the array any number of times (possibly zero):

        Choose any 3 consecutive elements from the array and flip all of them.
    Flipping an element means changing its value from 0 to 1, and from 1 to 0.

    Return the minimum number of operations required to make all elements in nums equal to 1. If it is impossible, return -1.



Example 1:

    Input: nums = [0,1,1,1,0,0]
    Output: 3
    Explanation:
        We can do the following operations:

        Choose the elements at indices 0, 1 and 2. The resulting array is nums = [1,0,0,1,0,0].
        Choose the elements at indices 1, 2 and 3. The resulting array is nums = [1,1,1,0,0,0].
        Choose the elements at indices 3, 4 and 5. The resulting array is nums = [1,1,1,1,1,1].

Example 2:

    Input: nums = [0,1,1,1]
    Output: -1
    Explanation:
        It is impossible to make all elements equal to 1.


Constraints:

    3 <= nums.length <= 105
    0 <= nums[i] <= 1

    
    if [0,0,0] -> 1
    if [1,0,0] -> 0
    if [0,1,0] -> 0
    if [0,0,1] -> 0
    if [1,1,1] -> 0

    if [0,0,0,0] -> [1,1,1,0] -> [1]
*/

fn solution_1(nums: Vec<i32>) -> i32 {
    let mut count = 0;

    let n = nums.len();
    let mut nums = nums.clone();

    for i in 0..n-2 {
        if nums[i] != 1 {
            nums[i] ^= 1;
            nums[i+1] ^= 1;
            nums[i+2] ^= 1;
            count += 1
        }
    }

    if nums[n-1] == 0 || nums[n-2] == 0 {
        return -1;
    }
    count
}


pub fn main() {
    println!(
        "{:?}",
        solution_1(vec![0,1,1,1,0,0])
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(true, true);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(true, true);
    }
    
    #[test]
    fn test_case_3() {
        assert_eq!(true, true);
    }
}

