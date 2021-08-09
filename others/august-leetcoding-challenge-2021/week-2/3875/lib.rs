pub struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let mut num1 = num1.bytes().rev();
        let mut num2 = num2.bytes().rev();
        let mut v = Vec::with_capacity(num1.len().max(num2.len()) + 1);
        let mut carry = false;
        loop {
            let n1 = num1.next().map(|u| u - b'0');
            let n2 = num2.next().map(|u| u - b'0');
            if n1.is_none() && n2.is_none() && !carry {
                break;
            }
            let d = if carry { 1 } else { 0 } + n1.unwrap_or_default() + n2.unwrap_or_default();
            carry = d > 9;
            v.push((b'0' + d % 10) as char);
        }
        v.iter().rev().collect()
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
