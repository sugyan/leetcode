pub struct Solution {}

impl Solution {
    pub fn largest_time_from_digits(a: Vec<i32>) -> String {
        let mut a = a;
        let mut v: Vec<i32> = Vec::new();
        let mut answers: Vec<(i32, i32)> = Vec::new();
        Solution::helper(&mut a, &mut v, &mut answers);
        if let Some(max) = answers.iter().max() {
            format!("{:02}:{:02}", max.0, max.1)
        } else {
            String::new()
        }
    }
    fn helper(a: &mut [i32], v: &mut Vec<i32>, answers: &mut Vec<(i32, i32)>) {
        if a.is_empty() {
            let (h, m) = (v[0] * 10 + v[1], v[2] * 10 + v[3]);
            if h < 24 && m < 60 {
                answers.push((h, m));
            }
            return;
        }
        for i in 0..a.len() {
            v.push(a[i]);
            a.swap(0, i);
            Solution::helper(&mut a[1..], v, answers);
            a.swap(0, i);
            v.pop();
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
