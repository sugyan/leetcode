pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let c = cost
            .iter()
            .fold((0, 0), |acc, &x| (acc.1, x + acc.0.min(acc.1)));
        c.0.min(c.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
