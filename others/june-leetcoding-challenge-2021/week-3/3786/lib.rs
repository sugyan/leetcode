pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        (0..num_rows)
            .scan(Vec::with_capacity(num_rows as usize), |state, _| {
                (1..state.len())
                    .rev()
                    .for_each(|i| state[i] += state[i - 1]);
                state.push(1);
                Some(state.clone())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ],
            Solution::generate(5)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![vec![1]], Solution::generate(1));
    }
}
