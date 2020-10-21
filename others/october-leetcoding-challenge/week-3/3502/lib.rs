pub struct Solution {}

impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::new();
        for &asteroid in asteroids.iter() {
            answer.push(asteroid);
            while answer.len() > 1 && answer[answer.len() - 2] > 0 && answer[answer.len() - 1] < 0 {
                let l = answer.pop().unwrap();
                let r = answer.pop().unwrap();
                match r.cmp(&-l) {
                    std::cmp::Ordering::Less => answer.push(l),
                    std::cmp::Ordering::Equal => {}
                    std::cmp::Ordering::Greater => answer.push(r),
                }
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
        assert_eq!(vec![5, 10], Solution::asteroid_collision(vec![5, 10, -5]));
    }

    #[test]
    fn example_2() {
        let v: Vec<i32> = Vec::new();
        assert_eq!(v, Solution::asteroid_collision(vec![8, -8]));
    }

    #[test]
    fn example_3() {
        assert_eq!(vec![10], Solution::asteroid_collision(vec![10, 2, -5]));
    }

    #[test]
    fn example_4() {
        assert_eq!(
            vec![-2, -1, 1, 2],
            Solution::asteroid_collision(vec![-2, -1, 1, 2])
        );
    }
}
