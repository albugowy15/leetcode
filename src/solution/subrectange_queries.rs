struct SubrectangleQueries {
    rect: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SubrectangleQueries {
    fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { rect: rectangle }
    }

    fn update_subrectangle(&mut self, row1: i32, col1: i32, row2: i32, col2: i32, new_value: i32) {
        for row in row1..=row2 {
            for col in col1..=col2 {
                self.rect[row as usize][col as usize] = new_value;
            }
        }
    }

    fn get_value(&self, row: i32, col: i32) -> i32 {
        self.rect[row as usize][col as usize]
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */

#[cfg(test)]
mod tests {
    use super::*; // Import the parent module's items

    #[test]
    fn test_subrectangle_queries() {
        // This test follows the exact sequence provided in the problem explanation.
        // 1. Initialize the structure.
        let initial_rectangle = vec![vec![1, 2, 1], vec![4, 3, 4], vec![3, 2, 1], vec![1, 1, 1]];
        let mut subrectangle = SubrectangleQueries::new(initial_rectangle);

        // 2. "getValue(0, 2)" -> Expected: 1
        assert_eq!(subrectangle.get_value(0, 2), 1, "Initial getValue(0, 2)");

        // 3. "updateSubrectangle(0, 0, 3, 2, 5)"
        subrectangle.update_subrectangle(0, 0, 3, 2, 5);

        // 4. "getValue(0, 2)" -> Expected: 5
        assert_eq!(
            subrectangle.get_value(0, 2),
            5,
            "After first update, getValue(0, 2)"
        );

        // 5. "getValue(3, 1)" -> Expected: 5
        assert_eq!(
            subrectangle.get_value(3, 1),
            5,
            "After first update, getValue(3, 1)"
        );

        // 6. "updateSubrectangle(3, 0, 3, 2, 10)"
        subrectangle.update_subrectangle(3, 0, 3, 2, 10);

        // 7. "getValue(3, 1)" -> Expected: 10
        assert_eq!(
            subrectangle.get_value(3, 1),
            10,
            "After second update, getValue(3, 1)"
        );

        // 8. "getValue(0, 2)" -> Expected: 5
        assert_eq!(
            subrectangle.get_value(0, 2),
            5,
            "After second update, getValue(0, 2)"
        );
    }
}
