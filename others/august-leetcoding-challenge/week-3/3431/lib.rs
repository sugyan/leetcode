pub struct Solution {}

impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        let mut answer = a;
        let (mut l, mut r) = (0, answer.len() - 1);
        while l < r {
            while l < r && answer[l] % 2 == 0 {
                l += 1;
            }
            answer.swap(l, r);
            r -= 1;
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![4, 2, 1, 3],
            Solution::sort_array_by_parity(vec![3, 1, 2, 4])
        );
    }
}
