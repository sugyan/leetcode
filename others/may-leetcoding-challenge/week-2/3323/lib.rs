pub struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        if coordinates.len() < 3 {
            return true;
        }
        let r = (
            coordinates[1][0] - coordinates[0][0],
            coordinates[1][1] - coordinates[0][1],
        );
        for i in 2..coordinates.len() {
            if (coordinates[i][0] - coordinates[0][0]) * r.1
                != (coordinates[i][1] - coordinates[0][1]) * r.0
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            true,
            Solution::check_straight_line(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![6, 7]
            ])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            false,
            Solution::check_straight_line(vec![
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![4, 5],
                vec![5, 6],
                vec![7, 7]
            ])
        );
    }
}
