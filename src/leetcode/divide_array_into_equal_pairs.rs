/*

2206. Divide Array Into Equal Pairs
Easy - Topics - Companies - Hint

You are given an integer array nums consisting of 2 * n integers.

    You need to divide nums into n pairs such that:

    Each element belongs to exactly one pair.
    The elements present in a pair are equal.
    Return true if nums can be divided into n pairs, otherwise return false.



Example 1:

    Input: nums = [3,2,3,2,2,2]
    Output: true
    Explanation: 
        There are 6 elements in nums, so they should be divided into 6 / 2 = 3 pairs.
        If nums is divided into the pairs (2, 2), (3, 3), and (2, 2), it will satisfy all the conditions.

Example 2:

    Input: nums = [1,2,3,4]
    Output: false
    Explanation: 
        There is no way to divide nums into 4 / 2 = 2 pairs such that the pairs satisfy every condition.


Constraints:

nums.length == 2 * n
1 <= n <= 500
1 <= nums[i] <= 500

*/

fn solution_1(nums: Vec<i32>) -> bool {
    /*  Let's use Bit Manuplation like XOR Operator 
        Use XOR manipulation to check if all elements cancel each other out
    */
    let n = nums.len();
    let mut result = nums[0];
    for index in 1..n {
        result ^= nums[index];
    }
    result == 0
}

use std::collections::HashMap;

fn solution_2(nums: Vec<i32>) -> bool {
    let mut count_map = HashMap::new();

    for num in nums {
        *count_map.entry(num).or_insert(0) +=1;
    }

    // println!("{:?}", count_map);

    for count in count_map.values() {
        if count %2 != 0 {
            return false;
        }
    }
    true
}


pub fn main() {
    let input_1 = vec![3,2,3,2,2,2];
    println!(
        "This is Solution 1 ->  result is {:?}",
        solution_1(input_1)
    );
    let input_2 = vec![3,2,3,2,2,2];
    println!(
        "This is Solution 2 ->  result is {:?}",
        solution_2(input_2)
    )
}


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn test_case_1() {
        assert_eq!(solution_1(vec![1, 2, 3, 3, 2, 1]), true);
        assert_eq!(solution_2(vec![1, 2, 3, 3, 2, 1]), true);
    }
    
    #[test]
    fn test_case_2() {
        assert_eq!(solution_1(vec![1,2,3,4]), false);
        assert_eq!(solution_2(vec![1,2,3,4]), false);
    }

    #[test]
    fn test_case_3() {
        // let input = vec![9,9,19,10,9,12,2,12,3,3,11,5,8,4,13,6,2,11,9,19,11,15,9,17,15,12,5,14,12,16,18,16,10,3,8,9,16,20,2,4,16,12,11,14,20,16,2,18,17,20,3,13,16,17,1,1,11,20,20,4];
        // assert_eq!(solution_1(input), false);

        let input = vec![9,9,19,10,9,12,2,12,3,3,11,5,8,4,13,6,2,11,9,19,11,15,9,17,15,12,5,14,12,16,18,16,10,3,8,9,16,20,2,4,16,12,11,14,20,16,2,18,17,20,3,13,16,17,1,1,11,20,20,4];
        assert_eq!(solution_2(input), false);
        // assert_eq!(solution_2(input), false);
    }
}