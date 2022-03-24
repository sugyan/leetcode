pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort();
        let (mut lo, mut hi) = (0, people.len() - 1);
        let mut answer = 0;
        while lo <= hi {
            answer += 1;
            if people[lo] + people[hi] <= limit {
                lo += 1;
            }
            if hi == 0 {
                break;
            }
            hi -= 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::num_rescue_boats(vec![1, 2], 3));
    }

    #[test]
    fn example_2() {
        assert_eq!(3, Solution::num_rescue_boats(vec![3, 2, 2, 1], 3));
    }

    #[test]
    fn example_3() {
        assert_eq!(4, Solution::num_rescue_boats(vec![3, 5, 3, 4], 5));
    }
}
