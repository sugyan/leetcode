pub struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes().iter().rev().collect::<Vec<_>>();
        let num2 = num2.as_bytes().iter().rev().collect::<Vec<_>>();
        let len = num1.len().max(num2.len()) + 1;
        let mut v = Vec::with_capacity(len);
        let mut carry = false;
        for i in 0..len {
            let d = if carry { 1 } else { 0 }
                + if i < num1.len() { num1[i] - b'0' } else { 0 }
                + if i < num2.len() { num2[i] - b'0' } else { 0 };
            carry = d > 9;
            v.push(d % 10);
        }
        while v.len() > 1 && v.last() == Some(&0) {
            v.pop();
        }
        v.iter().rev().map(|u| (u + b'0') as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "134",
            Solution::add_strings(String::from("11"), String::from("123"))
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "533",
            Solution::add_strings(String::from("456"), String::from("77"))
        )
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "0",
            Solution::add_strings(String::from("0"), String::from("0"))
        )
    }
}
