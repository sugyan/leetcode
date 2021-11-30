pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }
        let cols = matrix[0].len();
        let mut h = vec![0; cols + 1];
        let mut answer = 0;
        for row in &matrix {
            let mut stack = Vec::new();
            for j in 0..=cols {
                if j < cols {
                    h[j] = if row[j] == '1' { h[j] + 1 } else { 0 };
                }
                while let Some(&last) = stack.last() {
                    if h[j] < h[last] {
                        let height = h[stack.pop().unwrap()];
                        let width = match stack.last() {
                            Some(&l) => j - l - 1,
                            None => j,
                        };
                        let area = (height * width) as i32;
                        answer = answer.max(area)
                    } else {
                        break;
                    }
                }
                stack.push(j);
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
        assert_eq!(
            6,
            Solution::maximal_rectangle(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::maximal_rectangle(vec![]));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::maximal_rectangle(vec![vec!['0']]));
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::maximal_rectangle(vec![vec!['1']]));
    }

    #[test]
    fn example_5() {
        assert_eq!(0, Solution::maximal_rectangle(vec![vec!['0', '0']]));
    }
}
