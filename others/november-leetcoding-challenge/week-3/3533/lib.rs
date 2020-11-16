pub struct Solution {}

impl Solution {
    pub fn longest_mountain(a: Vec<i32>) -> i32 {
        let mut answer = 0;
        let mut state = 0;
        let mut start = 0;
        for i in 1..a.len() {
            match a[i - 1].cmp(&a[i]) {
                std::cmp::Ordering::Less => {
                    if state == 2 {
                        answer = std::cmp::max(answer, i - start);
                    }
                    if state != 1 {
                        state = 1;
                        start = i - 1;
                    }
                }
                std::cmp::Ordering::Equal => {
                    if state == 2 {
                        answer = std::cmp::max(answer, i - start);
                    }
                    state = 0;
                }
                std::cmp::Ordering::Greater => {
                    if state == 1 {
                        state = 2;
                    }
                }
            }
        }
        if state == 2 {
            answer = std::cmp::max(answer, a.len() - start);
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(5, Solution::longest_mountain(vec![2, 1, 4, 7, 3, 2, 5]));
    }

    #[test]
    fn example_2() {
        assert_eq!(0, Solution::longest_mountain(vec![2, 2, 2]));
    }
}
