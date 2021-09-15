pub struct Solution;

impl Solution {
    pub fn max_turbulence_size(arr: Vec<i32>) -> i32 {
        let v = arr
            .windows(2)
            .map(|w| (w[0] - w[1]).signum())
            .collect::<Vec<_>>();
        let mut anchor = 0;
        let mut answer = 1;
        for i in 0..v.len() {
            if v[i] == 0 {
                anchor = i + 1;
            } else if i == v.len() - 1 || v[i] * v[i + 1] != -1 {
                answer = answer.max(i - anchor + 2);
                anchor = i + 1;
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            5,
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::max_turbulence_size(vec![4, 8, 12, 16]));
    }

    #[test]
    fn example_3() {
        assert_eq!(1, Solution::max_turbulence_size(vec![100]));
    }
}
