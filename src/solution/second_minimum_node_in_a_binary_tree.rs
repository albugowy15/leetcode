// Definition for a binary tree node.
use std::collections::VecDeque;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn from_vec(data: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() || data[0].is_none() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(data[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < data.len() {
            let node = queue.pop_front().unwrap();
            if let Some(val) = data[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
            if i < data.len() {
                if let Some(val) = data[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }

        Some(root)
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use crate::solution::Solution;

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, smallest: i32, second_smallest: &mut i32) {
    match root {
        Some(node) => {
            let value = node.borrow().val;
            if value > smallest {
                if *second_smallest == -1 {
                    *second_smallest = value;
                } else {
                    *second_smallest = std::cmp::min(value, *second_smallest);
                }
            }

            if let Some(left_node) = &node.borrow().left {
                traverse(Some(left_node.clone()), smallest, second_smallest);
            }
            if let Some(right_node) = &node.borrow().right {
                traverse(Some(right_node.clone()), smallest, second_smallest);
            }
        }
        None => return,
    }
}

impl Solution {
    pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(ref node) => {
                let smallest = node.borrow().val;
                let mut second_smallest = -1;
                traverse(root, smallest, &mut second_smallest);
                second_smallest
            }
            None => -1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCase {
        input: Vec<Option<i32>>,
        expected: i32,
    }

    #[test]
    fn test_find_second_minimum_value() {
        let test_cases = vec![
            TestCase {
                input: vec![Some(2), Some(2), Some(5), None, None, Some(5), Some(7)],
                expected: 5,
            },
            TestCase {
                input: vec![Some(2), Some(2), Some(2)],
                expected: -1,
            },
            TestCase {
                input: vec![
                    Some(1),
                    Some(1),
                    Some(3),
                    Some(1),
                    Some(1),
                    Some(3),
                    Some(4),
                    Some(3),
                    Some(1),
                    Some(1),
                    Some(1),
                    Some(3),
                    Some(8),
                    Some(4),
                    Some(8),
                    Some(3),
                    Some(3),
                    Some(1),
                    Some(6),
                    Some(2),
                    Some(1),
                ],
                expected: 2,
            },
        ];
        for (idx, tc) in test_cases.into_iter().enumerate() {
            let nodes = TreeNode::from_vec(tc.input);
            assert_eq!(
                Solution::find_second_minimum_value(nodes),
                tc.expected,
                "Test case #{}",
                idx + 1
            );
        }
    }
}
