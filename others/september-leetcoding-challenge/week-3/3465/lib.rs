pub struct Solution {}

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut v: Vec<String> = vec![String::from("12")];
        while let Some(n) = v.last() {
            if n.len() >= 9 {
                break;
            }
            if let Some(last) = n.chars().last() {
                let next = match last {
                    '9' => (0..=n.len()).map(|i| (b'1' + i as u8) as char).collect(),
                    _ => n.chars().map(|c| (c as u8 + 1) as char).collect(),
                };
                v.push(next);
            }
        }
        v.iter()
            .map(|s| s.parse().unwrap())
            .filter(|&n| low <= n && n <= high)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![123, 234], Solution::sequential_digits(100, 300));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![1234, 2345, 3456, 4567, 5678, 6789, 12345],
            Solution::sequential_digits(1000, 13000)
        );
    }
}
