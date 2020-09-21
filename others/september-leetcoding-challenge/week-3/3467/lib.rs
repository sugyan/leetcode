pub struct Solution {}

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut v: Vec<i32> = vec![0; 1001];
        for trip in trips.iter() {
            v[trip[1] as usize] += trip[0];
            v[trip[2] as usize] -= trip[0];
        }
        let mut sum = 0;
        for passengers in v.iter() {
            sum += passengers;
            if sum > capacity {
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
            false,
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4)
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            true,
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            true,
            Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 5, 7]], 3)
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            true,
            Solution::car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11)
        );
    }
}
