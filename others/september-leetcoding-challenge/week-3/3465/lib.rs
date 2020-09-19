use std::collections::VecDeque;

pub struct Solution {}

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut vd: VecDeque<i32> = (1..=9).collect::<VecDeque<i32>>();
        let mut answer: Vec<i32> = Vec::new();
        while let Some(front) = vd.pop_front() {
            if low <= front && front <= high {
                answer.push(front);
            }
            let d = front % 10;
            if d < 9 {
                vd.push_back(front * 10 + d + 1);
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
        assert_eq!(vec![123, 234], Solution::sequential_digits(100, 300));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345],
            Solution::sequential_digits(1000, 13000)
        );
    }
}
