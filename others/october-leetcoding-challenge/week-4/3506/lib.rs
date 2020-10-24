pub struct Solution {}

impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, p: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }
        let mut tokens = tokens;
        tokens.sort_unstable();
        let mut sum = vec![0; tokens.len() + 1];
        for i in 0..tokens.len() {
            sum[i + 1] = sum[i] + tokens[i];
        }
        let (mut l, mut r) = (0, tokens.len() - 1);
        let mut p = p;
        let mut answer = 0;
        while l <= r {
            if p < tokens[l] {
                break;
            }
            let i = match sum.binary_search(&(p + sum[l])) {
                Ok(i) => i,
                Err(i) => i - 1,
            } - 1;
            answer = std::cmp::max(answer, std::cmp::min(r, i) - l + 1);
            p += tokens[r] - tokens[l];
            l += 1;
            if r > 0 {
                r -= 1;
            }
        }
        answer as i32
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
