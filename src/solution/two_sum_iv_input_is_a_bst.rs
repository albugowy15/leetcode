use crate::solution::Solution;
use crate::solution::second_minimum_node_in_a_binary_tree::TreeNode;

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

fn dfs(node: Option<Rc<RefCell<TreeNode>>>, k: i32, seen: &mut HashSet<i32>) -> bool {
    if let Some(node) = node {
        if seen.contains(&(k - node.borrow().val)) {
            return true;
        }
        seen.insert(node.borrow().val);
        return dfs(node.borrow().left.clone(), k, seen)
            || dfs(node.borrow().right.clone(), k, seen);
    }
    return false;
}

impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        return dfs(root, k, &mut seen);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct TestCaseInput {
        root: Vec<Option<i32>>,
        k: i32,
    }
    struct TestCase {
        input: TestCaseInput,
        output: bool,
    }
    #[test]
    fn test_find_target() {
        let test_cases = vec![
            TestCase {
                input: TestCaseInput {
                    root: vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)],
                    k: 9,
                },
                output: true,
            },
            TestCase {
                input: TestCaseInput {
                    root: vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)],
                    k: 28,
                },
                output: false,
            },
        ];
        for (idx, tc) in test_cases.into_iter().enumerate() {
            let nodes = TreeNode::from_vec(tc.input.root);
            assert_eq!(
                Solution::find_target(nodes, tc.input.k),
                tc.output,
                "Test case #{}",
                idx + 1
            );
        }
    }
}
