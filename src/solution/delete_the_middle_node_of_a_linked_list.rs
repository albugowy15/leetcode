// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn from_vec(values: Vec<i32>) -> Self {
        let mut head = None;
        for &val in values.iter().rev() {
            let mut node = Box::new(ListNode::new(val));
            node.next = head;
            head = Some(node);
        }
        return *head.unwrap();
    }
    fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }
        return result;
    }
}

use crate::solution::Solution;

impl Solution {
    pub fn delete_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return None;
        }

        let mut dummy = Box::new(ListNode { val: 0, next: head });
        let mut prev = &mut *dummy as *mut ListNode;
        let mut slow = unsafe { (*prev).next.as_mut().unwrap().as_mut() as *mut ListNode };
        let mut fast = slow;

        unsafe {
            while !fast.is_null() && (*fast).next.is_some() {
                prev = slow;
                slow = (*slow).next.as_mut().unwrap().as_mut() as *mut ListNode;
                fast = (*fast)
                    .next
                    .as_ref()
                    .and_then(|n| n.next.as_ref())
                    .map(|n| n.as_ref() as *const ListNode as *mut ListNode)
                    .unwrap_or(std::ptr::null_mut());
            }

            (*prev).next = (*slow).next.take();
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_middle_single_node() {
        // let head = create_list(vec![1]);
        let head = ListNode::from_vec(vec![1]);
        let result = Solution::delete_middle(Some(Box::new(head)));
        assert_eq!(result, None);
    }

    #[test]
    fn test_delete_middle_two_nodes() {
        let head = Some(Box::new(ListNode::from_vec(vec![1, 2])));
        let result = Solution::delete_middle(head);
        let expected = Some(Box::new(ListNode::from_vec(vec![1])));
        assert_eq!(ListNode::to_vec(result), ListNode::to_vec(expected));
    }

    #[test]
    fn test_delete_middle_three_nodes() {
        let head = Some(Box::new(ListNode::from_vec(vec![1, 2, 3])));
        let result = Solution::delete_middle(head);
        let expected = Some(Box::new(ListNode::from_vec(vec![1, 3])));
        assert_eq!(ListNode::to_vec(result), ListNode::to_vec(expected));
    }

    #[test]
    fn test_delete_middle_five_nodes() {
        let head = Some(Box::new(ListNode::from_vec(vec![1, 3, 4, 7, 1])));
        let result = Solution::delete_middle(head);
        let expected = Some(Box::new(ListNode::from_vec(vec![1, 3, 7, 1])));
        assert_eq!(ListNode::to_vec(result), ListNode::to_vec(expected));
    }

    #[test]
    fn test_delete_middle_six_nodes() {
        let head = Some(Box::new(ListNode::from_vec(vec![1, 2, 3, 4, 5, 6])));
        let result = Solution::delete_middle(head);
        let expected = Some(Box::new(ListNode::from_vec(vec![1, 2, 3, 5, 6])));
        assert_eq!(ListNode::to_vec(result), ListNode::to_vec(expected));
    }

    #[test]
    fn test_delete_middle_four_nodes() {
        let head = Some(Box::new(ListNode::from_vec(vec![2, 1, 3, 4])));
        let result = Solution::delete_middle(head);
        let expected = Some(Box::new(ListNode::from_vec(vec![2, 1, 4])));
        assert_eq!(ListNode::to_vec(result), ListNode::to_vec(expected));
    }
}
