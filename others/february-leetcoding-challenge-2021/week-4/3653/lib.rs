pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::with_capacity(pushed.len());
        let mut i = 0;
        for &n in &pushed {
            stack.push(n);
            while let Some(&last) = stack.last() {
                if last == popped[i] {
                    stack.pop();
                    i += 1;
                } else {
                    break;
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2])
        );
    }
}
