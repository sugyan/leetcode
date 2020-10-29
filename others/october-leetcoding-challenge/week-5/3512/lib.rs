pub struct Solution {}

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut d = 0;
        for &seat in seats.iter() {
            if seat == 0 {
                d += 1;
            } else {
                answer = std::cmp::max(answer, (d + 1) / 2);
                d = 0;
            }
        }
        answer = std::cmp::max(
            answer,
            seats.iter().take_while(|&seat| *seat == 0).count() as i32,
        );
        answer = std::cmp::max(
            answer,
            seats.iter().rev().take_while(|&seat| *seat == 0).count() as i32,
        );
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(2, Solution::max_dist_to_closest(vec![1, 0, 0, 0, 1, 0, 1]));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::max_dist_to_closest(vec![1, 0, 0, 0]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::max_dist_to_closest(vec![0, 1]));
    }
}
