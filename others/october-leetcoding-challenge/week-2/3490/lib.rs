pub struct Solution {}

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        let mut points = points;
        points.sort_unstable_by_key(|p| p[1]);
        let mut answer = 1;
        let mut max = points[0][1];
        for point in points.iter().skip(1) {
            if point[0] <= max {
                continue;
            }
            answer += 1;
            max = point[1];
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
            2,
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            4,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]])
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            2,
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]])
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(1, Solution::find_min_arrow_shots(vec![vec![1, 2]]));
    }
}
