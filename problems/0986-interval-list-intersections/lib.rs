pub struct Solution;

impl Solution {
    pub fn interval_intersection(
        first_list: Vec<Vec<i32>>,
        second_list: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let mut answer = Vec::new();
        let mut idx = (0, 0);
        while idx.0 < first_list.len() && idx.1 < second_list.len() {
            let lo = first_list[idx.0][0].max(second_list[idx.1][0]);
            let hi = first_list[idx.0][1].min(second_list[idx.1][1]);
            if lo <= hi {
                answer.push(vec![lo, hi]);
            }
            if first_list[idx.0][1] < second_list[idx.1][1] {
                idx.0 += 1;
            } else {
                idx.1 += 1;
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

    #[test]
    fn example_2() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::interval_intersection(vec![vec![1, 3], vec![5, 9]], vec![])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::interval_intersection(vec![], vec![vec![4, 8], vec![10, 12]])
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            vec![vec![3, 7]],
            Solution::interval_intersection(vec![vec![1, 7]], vec![vec![3, 10]])
        );
    }
}
