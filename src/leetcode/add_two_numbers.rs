/*
2. Add Two Numbers
Solved - Medium - Topics - Companies

    You are given two non-empty linked lists representing two non-negative integers. 
        The digits are stored in reverse order, and each of their nodes contains a single digit. 
            Add the two numbers and return the sum as a linked list.

    You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example 1:


    Input: l1 = [2,4,3], l2 = [5,6,4]
    Output: [7,0,8]
    
    Explanation: 342 + 465 = 807.

Example 2:

    Input: l1 = [0], l2 = [0]
    Output: [0]

Example 3:

    Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
    Output: [8,9,9,9,0,0,0,1]


Constraints:

The number of nodes in each linked list is in the range [1, 100].
0 <= Node.val <= 9
It is guaranteed that the list represents a number that does not have leading zeros.
*/



// struct ListNode {
//     val: i32,
//     next: Option<Box<ListNode>>
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode {
//         next: None,
//         val
//         }
//     }
// }

// fn solution_1(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
//     let mut l1 = l1;
//     let mut l2 = l2;

//     let mut dummy_head: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
//     let mut current = &mut dummy_head;

//     let mut carry = 0;

//     while l1.is_some() || l2.is_some() || carry !=0 {
//         let val1 = if let Some(node) = l1 {
//             l1 = node.next;
//             node.val
//         }else {
//             0
//         };

//         let val2 = if let Some(node) = l2 {
//             l2 = node.next;
//             node.val
//         } else {
//             0
//         };

//         let sum = val1 + val2 + carry;
//         carry = sum /10;
//         current.next = Some(Box::new(ListNode::new(sum % 10)));

//     }
//     dummy_head.next

// }

// pub fn main() {
//     let l1 = Some(Box::new(ListNode {
//         val: 2,
//         next: Some(Box::new(ListNode {
//             val: 4,
//             next: Some(Box::new(ListNode {
//                 val: 3,
//                 next: None,
//             })),
//         })),
//     }));

//     let l2: = Some(Box::new(ListNode { 
//         val: 5, 
//         next: Some(Box::new(ListNode { 
//             val: 6,
//             next: Some(Box::new(ListNode { 
//                 val: 4,
//                 next: None,
//             })),
//         })),
//     }));

//     println!(
//         "This is Solution 1 -> Result for Input 1 is {:?}",
//         solution_1(l1, l2)
//     );
// }

// #[cfg(test)]
// mod test {

//     #[test]
//     fn test_case_1() {
//         assert_eq!(solution_1(), )
//     }
// }