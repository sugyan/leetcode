pub struct Solution;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let n = mat[0].len();
        let mut added = vec![false; mat.len()];
        let mut answer = Vec::new();
        for i in 0..n {
            for (j, row) in mat.iter().enumerate() {
                if row[i] == 0 && !added[j] {
                    answer.push(j as i32);
                    if answer.len() == k as usize {
                        return answer;
                    }
                    added[j] = true;
                }
            }
        }
        answer.extend(
            added
                .iter()
                .enumerate()
                .filter_map(|(i, &b)| if b { None } else { Some(i as i32) })
                .take(k as usize - answer.len()),
        );
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![2, 0, 3],
            Solution::k_weakest_rows(
                vec![
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 0],
                    vec![1, 0, 0, 0, 0],
                    vec![1, 1, 0, 0, 0],
                    vec![1, 1, 1, 1, 1]
                ],
                3
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![0, 2],
            Solution::k_weakest_rows(
                vec![
                    vec![1, 0, 0, 0],
                    vec![1, 1, 1, 1],
                    vec![1, 0, 0, 0],
                    vec![1, 0, 0, 0]
                ],
                2
            )
        );
    }
}
