pub struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let mut tokens = tokens;
        tokens.sort_unstable();
        let (mut l, mut r) = (0, tokens.len() - 1);
        let mut p = p;
        let mut score = 0;
        let mut answer = 0;
        while l <= r && (p >= tokens[l] || score > 0) {
            while l <= r && p >= tokens[l] {
                p -= tokens[l];
                l += 1;
                score += 1;
            }
            answer = std::cmp::max(answer, score);
            if l <= r && score > 0 {
                p += tokens[r];
                r -= 1;
                score -= 1;
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(0, Solution::bag_of_tokens_score(vec![100], 50));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::bag_of_tokens_score(vec![100, 200], 150));
    }

    #[test]
    fn example_3() {
        assert_eq!(
            2,
            Solution::bag_of_tokens_score(vec![100, 200, 300, 400], 200)
        );
    }
}
