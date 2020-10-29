pub struct Solution {}

impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        let mut answer = None;
        let mut d = 0;
        for i in 0..=seats.len() {
            if i < seats.len() && seats[i] == 0 {
                d += 1;
            } else {
                if let Some(a) = answer {
                    answer = Some(std::cmp::max(
                        a,
                        if i == seats.len() { d } else { (d + 1) / 2 },
                    ));
                } else {
                    answer = Some(d);
                }
                d = 0;
            }
        }
        answer.unwrap()
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
