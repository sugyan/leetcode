pub struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::stone_game(vec![5, 3, 4, 5]));
    }
}
