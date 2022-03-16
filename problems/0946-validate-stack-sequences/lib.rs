pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut iter = popped.iter().peekable();
        for n in &pushed {
            stack.push(n);
            while !stack.is_empty() && stack.last() == iter.peek() {
                stack.pop();
                iter.next();
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1])
        );
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2])
        );
    }
}
