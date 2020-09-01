pub struct Solution {}

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let mut a = a;
        let mut answers: Vec<(i32, i32)> = Vec::new();
        Solution::helper(&mut a, 0, &mut answers);
        if let Some(max) = answers.iter().max() {
            format!("{:02}:{:02}", max.0, max.1)
        } else {
            String::new()
        }
    }
    fn helper(a: &mut [i32], i: usize, answers: &mut Vec<(i32, i32)>) {
        if i == 4 {
            let (h, m) = (a[0] * 10 + a[1], a[2] * 10 + a[3]);
            if h < 24 && m < 60 {
                answers.push((h, m));
            }
            return;
        }
        for j in i..4 {
            a.swap(i, j);
            Solution::helper(a, i + 1, answers);
            a.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "23:41",
            Solution::largest_time_from_digits(vec![1, 2, 3, 4])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!("", Solution::largest_time_from_digits(vec![5, 5, 5, 5]));
    }
}
