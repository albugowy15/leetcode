use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    flat: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let mut stack: Vec<NestedInteger> = nested_list.into_iter().rev().collect();
        let mut flat = VecDeque::new();

        while let Some(item) = stack.pop() {
            match item {
                NestedInteger::Int(val) => flat.push_back(val),
                NestedInteger::List(list) => {
                    for nested in list.into_iter().rev() {
                        stack.push(nested);
                    }
                }
            }
        }

        NestedIterator { flat }
    }

    fn next(&mut self) -> i32 {
        self.flat.pop_front().unwrap()
    }

    fn has_next(&self) -> bool {
        !self.flat.is_empty()
    }
}
