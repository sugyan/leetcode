use std::collections::{HashMap, HashSet};

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
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                hm.entry(email).or_insert_with(Vec::new).push(i);
            }
        }
        let mut uf = UnionFind::new(accounts.len());
        for v in hm.values() {
            for i in 1..v.len() {
                uf.union(v[0], v[i]);
            }
        }
        let mut emails = vec![HashSet::new(); accounts.len()];
        for (i, account) in accounts.iter().enumerate() {
            emails[uf.find(i)].extend(account.iter().skip(1));
        }
        let mut answer = Vec::new();
        for (i, hs) in emails.iter().enumerate() {
            if hs.is_empty() {
                continue;
            }
            let mut v = hs.iter().collect::<Vec<_>>();
            v.sort();
            let mut account = vec![accounts[i][0].clone()];
            account.extend(v.iter().map(|s| s.to_string()));
            answer.push(account);
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
