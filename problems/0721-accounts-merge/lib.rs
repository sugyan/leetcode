use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }
    fn union(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        self.parent[y] = x
    }
    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}

pub struct Solution;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut hm = HashMap::new();
        let mut uf = UnionFind::new(accounts.len());
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                if let Some(&j) = hm.get(email) {
                    uf.union(i, j);
                } else {
                    hm.insert(email, i);
                }
            }
        }
        let mut components = HashMap::new();
        for (&email, &i) in &hm {
            components
                .entry(uf.find(i))
                .or_insert_with(Vec::new)
                .push(email.clone());
        }
        let mut answer = Vec::new();
        for (&i, emails) in components.iter_mut() {
            let mut merged = vec![accounts[i][0].clone()];
            emails.sort();
            merged.extend(emails.iter().cloned());
            answer.push(merged);
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::accounts_merge(
            vec![
                vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
                vec!["John", "johnsmith@mail.com", "john00@mail.com"],
                vec!["Mary", "mary@mail.com"],
                vec!["John", "johnnybravo@mail.com"],
            ]
            .into_iter()
            .map(|v| v.into_iter().map(String::from).collect())
            .collect(),
        );
        ret.sort();
        assert_eq!(
            vec![
                vec![
                    "John",
                    "john00@mail.com",
                    "john_newyork@mail.com",
                    "johnsmith@mail.com"
                ],
                vec!["John", "johnnybravo@mail.com"],
                vec!["Mary", "mary@mail.com"],
            ],
            ret
        );
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::accounts_merge(
            vec![
                vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
                vec!["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
                vec!["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
                vec!["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
                vec!["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
            ]
            .into_iter()
            .map(|v| v.into_iter().map(String::from).collect())
            .collect(),
        );
        ret.sort();
        assert_eq!(
            vec![
                vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
                vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
                vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
                vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
                vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            ],
            ret
        );
    }
}
