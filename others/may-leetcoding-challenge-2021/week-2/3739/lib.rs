pub struct Solution;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        (0..=k as usize)
            .scan(card_points[0..k as usize].iter().sum(), |state, i| {
                if i > 0 {
                    *state += card_points[card_points.len() - i] - card_points[k as usize - i];
                }
                Some(*state)
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(12, Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3));
    }

    #[test]
    fn example_2() {
        assert_eq!(4, Solution::max_score(vec![2, 2, 2], 2));
    }

    #[test]
    fn example_3() {
        assert_eq!(55, Solution::max_score(vec![9, 7, 7, 9, 7, 7, 9], 7));
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::max_score(vec![1, 1000, 1], 1));
    }

    #[test]
    fn example_5() {
        assert_eq!(
            202,
            Solution::max_score(vec![1, 79, 80, 1, 1, 1, 200, 1], 3)
        );
    }
}
