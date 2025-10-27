#[allow(dead_code)]
struct ExamTracker {
    times: Vec<i32>,
    prefix_sum: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl ExamTracker {
    fn new() -> Self {
        return ExamTracker {
            times: Vec::new(),
            prefix_sum: Vec::new(),
        };
    }

    fn record(&mut self, time: i32, score: i32) {
        self.times.push(time);
        if self.prefix_sum.is_empty() {
            self.prefix_sum.push(score as i64);
        } else {
            let last = self.prefix_sum[self.prefix_sum.len() - 1];
            self.prefix_sum.push(last + score as i64);
        }
    }

    fn total_score(&self, start_time: i32, end_time: i32) -> i64 {
        let start = self.lower_bound(start_time);
        let end = self.upper_bound(end_time);

        if start > end || start == self.times.len() as i32 || end == -1 {
            return 0;
        }

        let mut total: i64 = self.prefix_sum[end as usize];
        if start > 0 {
            total -= self.prefix_sum[(start - 1) as usize];
        }

        return total;
    }

    fn lower_bound(&self, start_time: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (self.times.len() as i32) - 1;
        let mut ans: i32 = self.times.len() as i32;

        while low <= high {
            let mid = (low + high) / 2;
            if self.times[mid as usize] >= start_time {
                ans = mid;
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }

        return ans;
    }
    fn upper_bound(&self, end_time: i32) -> i32 {
        let mut low = 0i32;
        let mut high = (self.times.len() as i32) - 1;
        let mut ans: i32 = -1;

        while low <= high {
            let mid = (low + high) / 2;
            if self.times[mid as usize] <= end_time {
                ans = mid;
                low = mid + 1;
            } else {
                high = mid - 1;
            }
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exam_tracker() {
        let mut exam_tracker = ExamTracker::new();
        exam_tracker.record(1, 98); // Alice takes a new exam at time 1, scoring 98.
        assert_eq!(exam_tracker.total_score(1, 1), 98); // Between time 1 and time 1, Alice took 1 exam at time 1, scoring 98. The total score is 98.
        exam_tracker.record(5, 99); // Alice takes a new exam at time 5, scoring 99.
        assert_eq!(exam_tracker.total_score(1, 3), 98); // Between time 1 and time 3, Alice took 1 exam at time 1, scoring 98. The total score is 98.
        assert_eq!(exam_tracker.total_score(1, 5), 197); // Between time 1 and time 5, Alice took 2 exams at time 1 and 5, scoring 98 and 99. The total score is 98 + 99 = 197.
        assert_eq!(exam_tracker.total_score(3, 4), 0); // Alice did not take any exam between time 3 and time 4. Therefore, the answer is 0.
        assert_eq!(exam_tracker.total_score(2, 5), 99); // Between time 2 and time 5, Alice took 1 exam at time 5, scoring 99. The total score is 99.
    }
}
