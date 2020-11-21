pub struct Solution {}

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let n = n.to_string();
        let mut digits: Vec<char> = digits.iter().map(|s| s.chars().next().unwrap()).collect();
        digits.sort_unstable();
        let mut answer = 0;
        for i in 0..n.len() - 1 {
            answer += digits.len().pow(i as u32 + 1) as i32;
        }
        for (i, c) in n.chars().enumerate() {
            match digits.binary_search(&c) {
                Ok(j) => {
                    answer += (j * digits.len().pow((n.len() - i - 1) as u32)) as i32;
                    if i == n.len() - 1 {
                        answer += 1;
                    }
                }
                Err(j) => {
                    answer += (j * digits.len().pow((n.len() - i - 1) as u32)) as i32;
                    break;
                }
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
        assert_eq!(
            20,
            Solution::at_most_n_given_digit_set(
                vec![
                    String::from("1"),
                    String::from("3"),
                    String::from("5"),
                    String::from("7")
                ],
                100
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            29523,
            Solution::at_most_n_given_digit_set(
                vec![String::from("1"), String::from("4"), String::from("9")],
                1000000000
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            1,
            Solution::at_most_n_given_digit_set(vec![String::from("7")], 8)
        );
    }
}
