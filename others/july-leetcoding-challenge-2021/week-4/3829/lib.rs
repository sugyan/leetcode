pub struct Solution;

impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut answer = Vec::with_capacity(n as usize);
        answer.push(1);
        while answer.len() < n as usize {
            answer = answer
                .iter()
                .map(|m| m * 2 - 1)
                .chain(answer.iter().map(|m| m * 2))
                .filter(|&m| m <= n)
                .collect();
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![1, 3, 2, 4], Solution::beautiful_array(4));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1, 5, 3, 2, 4], Solution::beautiful_array(5));
    }
}
