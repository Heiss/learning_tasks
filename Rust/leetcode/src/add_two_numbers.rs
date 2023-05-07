// https://leetcode.com/problems/add-two-numbers/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let mut result = None;
        for &val in vec.iter().rev() {
            let mut node = ListNode::new(val);
            node.next = result;
            result = Some(Box::new(node));
        }
        result
    }
}

struct Solution;

impl Solution {
    /// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
    /// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(node), None) | (None, Some(node)) => Some(node),
            (Some(left_node), Some(right_node)) => {
                let mut new_val = left_node.val + right_node.val;
                let carry = (new_val >= 10)
                    .then(|| Box::new(ListNode::new(1)))
                    .or_else(|| None);
                new_val %= 10i32;

                let mut result = ListNode::new(new_val);
                result.next = Self::add_two_numbers(
                    Self::add_two_numbers(left_node.next, carry),
                    right_node.next,
                );
                Some(Box::new(result))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let l1 = ListNode::from_vec(vec![2, 4, 3]);
        let l2 = ListNode::from_vec(vec![5, 6, 4]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, ListNode::from_vec(vec![7, 0, 8]));
    }

    #[test]
    fn test_2() {
        let l1 = ListNode::from_vec(vec![0]);
        let l2 = ListNode::from_vec(vec![0]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, ListNode::from_vec(vec![0]));
    }

    #[test]
    fn test_3() {
        let l1 = ListNode::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_vec(vec![9, 9, 9, 9]);
        let result = Solution::add_two_numbers(l1, l2);
        assert_eq!(result, ListNode::from_vec(vec![8, 9, 9, 9, 0, 0, 0, 1]));
    }
}
