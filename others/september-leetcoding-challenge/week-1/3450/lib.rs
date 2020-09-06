pub struct Solution {}

impl Solution {
    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        let mut answer = 0;
        let size = a.len() as i32;
        for i in -size + 1..size {
            for j in -size + 1..size {
                let mut overlap = 0;
                for k in std::cmp::max(0, i)..std::cmp::min(size, size + i) {
                    for l in std::cmp::max(0, j)..std::cmp::min(size, size + j) {
                        if a[k as usize][l as usize] == 1
                            && b[(k - i) as usize][(l - j) as usize] == 1
                        {
                            overlap += 1;
                        }
                    }
                }
                answer = std::cmp::max(answer, overlap);
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
            3,
            Solution::largest_overlap(
                vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]]
            )
        );
    }
}
