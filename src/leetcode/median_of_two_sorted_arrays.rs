/* 

4. Median of Two Sorted Arrays

Hard - Topics - Companies

    Given two sorted arrays nums1 and nums2 of size m and n respectively, 
        return the median of the two sorted arrays.

    The overall run time complexity should be O(log (m+n)).


Example 1:

    Input: nums1 = [1,3], nums2 = [2]
    Output: 2.00000
    Explanation: 
        merged array = [1,2,3] and median is 2.

Example 2:

    Input: nums1 = [1,2], nums2 = [3,4]
    Output: 2.50000
    Explanation: 
        merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.


Constraints:

    nums1.length == m
    nums2.length == n
    0 <= m <= 1000
    0 <= n <= 1000
    1 <= m + n <= 2000
    -106 <= nums1[i], nums2[i] <= 106

*/

// fn solution_1(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
//     num1_total = (nums1.iter().sum() / nums1.len()) as f64;
//     num2_total = nums2.iter().sum() / nums2.len() as f64;
//     ((num1_total + num2_total) / 2) as f64
// }


pub fn main() {
    // let a = vec![1,3];
    // let b =  vec![2]; 

    // println!(
    //     "This is Solution 1 ->  result is {:?}",
    //     solution_1(a,b)
    // );

}