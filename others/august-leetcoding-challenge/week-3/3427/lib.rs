pub struct Solution {}

impl Solution {
    pub fn distribute_candies(candies: i32, num_people: i32) -> Vec<i32> {
        let mut candies = candies;
        let mut answer = vec![0; num_people as usize];
        for i in 0 as i32.. {
            answer[(i % num_people) as usize] += std::cmp::min(candies, i + 1);
            candies -= i + 1;
            if candies <= 0 {
                break;
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
        assert_eq!(vec![1, 2, 3, 1], Solution::distribute_candies(7, 4));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![5, 2, 3], Solution::distribute_candies(10, 3));
    }
}
