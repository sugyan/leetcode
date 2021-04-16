pub struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        s.chars()
            .fold(Vec::<(char, usize)>::new(), |mut stack, c| {
                match stack.last_mut() {
                    Some(last) if last.0 == c => {
                        last.1 += 1;
                        if last.1 == k as usize {
                            stack.pop();
                        }
                    }
                    _ => stack.push((c, 1)),
                }
                stack
            })
            .iter()
            .flat_map(|&(c, n)| std::iter::repeat(c).take(n))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("abcd", Solution::remove_duplicates(String::from("abcd"), 2));
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "aa",
            Solution::remove_duplicates(String::from("deeedbbcccbdaa"), 3)
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            "ps",
            Solution::remove_duplicates(String::from("pbbcggttciiippooaais"), 2)
        );
    }
}
