pub struct Solution {}

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut v: Vec<String> = nums.iter().map(|&num| num.to_string()).collect();
        v.sort_by(|a: &String, b: &String| (b.clone() + a).cmp(&(a.clone() + b)));
        if v[0] == "0" {
            String::from("0")
        } else {
            v.join("")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("210", Solution::largest_number(vec![10, 2]));
    }

    #[test]
    fn example_2() {
        assert_eq!("9534330", Solution::largest_number(vec![3, 30, 34, 5, 9]));
    }
}
