pub struct Solution;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        wall.len() as i32
            - wall
                .iter()
                .fold(std::collections::HashMap::new(), |mut acc, row| {
                    row.iter()
                        .take(row.len() - 1)
                        .scan(0, |state, &x| {
                            *state += x;
                            Some(*state)
                        })
                        .for_each(|x| *acc.entry(x).or_default() += 1);
                    acc
                })
                .values()
                .max()
                .unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::least_bricks(vec![
                vec![1, 2, 2, 1],
                vec![3, 1, 2],
                vec![1, 3, 2],
                vec![2, 4],
                vec![3, 1, 2],
                vec![1, 3, 1, 1]
            ])
        );
    }
}
