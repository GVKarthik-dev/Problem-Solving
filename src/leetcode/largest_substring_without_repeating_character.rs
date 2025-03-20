/*

3. Longest Substring Without Repeating CharacteIrs
Medium - Topics - Companies - Hint

    Given a string s, find the length of the longest substring without duplicate characters.


Example 1:

    Input: s = "abcabcbb"
    Output: 3
    Explanation: 
        The answer is "abc", with the length of 3.

Example 2:

    Input: s = "bbbbb"
    Output: 1
    Explanation: The answer is "b", with the length of 1.

Example 3:

    Input: s = "pwwkew"
    Output: 3
    Explanation: 
        The answer is "wke", with the length of 3.

Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.


Constraints:

    0 <= s.length <= 5 * 104
    s consists of English letters, digits, symbols and spaces.

*/

use std::collections::HashSet;

fn solution_1(s: String) -> i32{
    let mut left = 0;
    let mut set: HashSet<char> = HashSet::new();
    let mut largest = 0;

    for (right, cha) in s.chars().enumerate() {
        // If we find a duplicate character, move the left pointer to remove it
        while set.contains(&cha) {
            set.remove(&s[left..left + 1].chars().next().unwrap()); // Using left to remove a single character
            left += 1;
        }

        // Add the current character to the set
        set.insert(cha);
        
        // Update the largest size of the substring
        largest = largest.max(right - left + 1);
    }

    largest as i32
}


pub fn main() {
    let input_1 = "abcbbac".to_owned();
    println!(
        "This is Solution 1 ->  result is {:?}",
        solution_1(input_1)
    );
}

#[cfg(test)]
pub mod test {

    use super::*;

    #[test]
    fn test_case_1() {

        assert_eq!(solution_1("abcabcbb".to_owned()), 3);
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