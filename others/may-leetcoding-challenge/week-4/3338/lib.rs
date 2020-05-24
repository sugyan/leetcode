pub struct Solution {}

impl Solution {
    pub fn interval_intersection(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut answer: Vec<Vec<i32>> = Vec::new();
        let (mut ia, mut ib) = (0, 0);
        while ia < a.len() && ib < b.len() {
            let l = std::cmp::max(a[ia][0], b[ib][0]);
            let r = std::cmp::min(a[ia][1], b[ib][1]);
            if l <= r {
                answer.push(vec![l, r]);
            }
            if a[ia][1] < b[ib][1] {
                ia += 1;
            } else {
                ib += 1;
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
            vec![
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25]
            ],
            Solution::interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]]
            )
        );
    }
}
