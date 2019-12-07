pub struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut answer = vec![vec![0; n as usize]; n as usize];
        let mut i = 1;
        for m in 0..(n as usize + 1) / 2 {
            let size = n as usize - 2 * m;
            if size == 1 {
                answer[m][m] = i;
            }
            for j in 0..size - 1 {
                answer[m][m + j] = i;
                i += 1;
            }
            for j in 0..size - 1 {
                answer[m + j][m + size - 1] = i;
                i += 1;
            }
            for j in 0..size - 1 {
                answer[m + size - 1][m + size - 1 - j] = i;
                i += 1;
            }
            for j in 0..size - 1 {
                answer[m + size - 1 - j][m] = i;
                i += 1;
            }
        }
        return answer;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            Solution::generate_matrix(3)
        );
    }
}
