pub struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let (mut lo, mut hi) = (0, piles.len() - 1);
        let mut score = 0;
        while lo < hi - 2 {
            if piles[lo] - piles[lo + 1].max(piles[hi]) > piles[hi] - piles[hi - 1].max(piles[lo]) {
                score += piles[lo];
                lo += 1;
            } else {
                score += piles[hi];
                hi -= 1;
            }
        }
        score + (piles[lo] - piles[hi]).abs() > 0
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
