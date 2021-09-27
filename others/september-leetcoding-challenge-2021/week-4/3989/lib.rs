use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        emails
            .iter()
            .map(|email| {
                let mut names = email.split('@').collect::<Vec<_>>();
                let local = names[0].replace('.', "");
                names[0] = local.split('+').next().unwrap();
                names.join("@")
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            2,
            Solution::num_unique_emails(
                vec![
                    "test.email+alex@leetcode.com",
                    "test.e.mail+bob.cathy@leetcode.com",
                    "testemail+david@lee.tcode.com"
                ]
                .into_iter()
                .map(String::from)
                .collect()
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            3,
            Solution::num_unique_emails(
                vec!["a@leetcode.com", "b@leetcode.com", "c@leetcode.com"]
                    .into_iter()
                    .map(String::from)
                    .collect()
            )
        );
    }
}
