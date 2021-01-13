pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort_unstable();
        let (mut l, mut r) = (0, people.len() - 1);
        let mut answer = 0;
        while l < r {
            if people[l] + people[r] <= limit {
                l += 1;
            }
            r -= 1;
            answer += 1;
        }
        answer + if l == r { 1 } else { 0 }
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
