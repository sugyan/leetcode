pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut answer = Vec::new();
        let mut v = Vec::new();
        Self::backtrack(n, &mut v, &mut answer);
        answer
    }
    fn backtrack(n: i32, v: &mut Vec<(i32, i32)>, answer: &mut Vec<Vec<String>>) {
        if v.len() == n as usize {
            answer.push(
                v.iter()
                    .map(|p| (0..n).map(|i| if i == p.1 { 'Q' } else { '.' }).collect())
                    .collect(),
            );
            return;
        }
        let i = v.len() as i32;
        for j in 0..n {
            if v.iter()
                .any(|p| p.1 == j || (p.0 - i).abs() == (p.1 - j).abs())
            {
                continue;
            }
            v.push((i, j));
            Self::backtrack(n, v, answer);
            v.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![
                vec![".Q..", "...Q", "Q...", "..Q."],
                vec!["..Q.", "Q...", "...Q", ".Q.."]
            ],
            Solution::solve_n_queens(4)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![vec!["Q"]], Solution::solve_n_queens(1));
    }
}
