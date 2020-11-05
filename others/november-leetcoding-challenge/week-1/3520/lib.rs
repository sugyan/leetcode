pub struct Solution {}

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let odd = position.iter().filter(|&p| p % 2 == 1).count();
        std::cmp::min(odd, position.len() - odd) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 2, 3]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::min_cost_to_move_chips(vec![2, 2, 2, 3, 3]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 1000000000]));
    }
}
