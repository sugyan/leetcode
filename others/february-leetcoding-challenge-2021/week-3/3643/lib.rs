pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut v: Vec<(i32, usize)> = Vec::new();
        let mut answer = 0;
        for (i, &h) in height.iter().enumerate() {
            for e in &v {
                answer = std::cmp::max(answer, std::cmp::min(h, e.0) * (i - e.1) as i32);
            }
            if let Some(&(last, _)) = v.last().or(Some(&(0, 0))) {
                if h > last {
                    v.push((h, i));
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
        assert_eq!(49, Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]));
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::max_area(vec![1, 1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(16, Solution::max_area(vec![4, 3, 2, 1, 4]));
    }

    #[test]
    fn example_4() {
        assert_eq!(2, Solution::max_area(vec![1, 2, 1]));
    }
}
