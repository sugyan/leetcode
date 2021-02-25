use std::collections::HashSet;

// This is the Master's API interface.
// You should not implement it, or speculate about its implementation
pub struct Master {
    secret: String,
    num_calls: usize,
}

impl Master {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            num_calls: 0,
        }
    }
    pub fn guess(&mut self, word: String) -> i32 {
        self.num_calls += 1;
        word.chars()
            .zip(self.secret.chars())
            .filter(|&cs| cs.0 == cs.1)
            .count() as i32
    }
}

pub struct Solution;

impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &mut Master) {
        let mut h = vec![vec![0; words.len()]; words.len()];
        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                h[i][j] = words[i]
                    .chars()
                    .zip(words[j].chars())
                    .filter(|&cs| cs.0 == cs.1)
                    .count();
                h[j][i] = h[i][j];
            }
        }
        let mut possible = (0..words.len()).collect::<Vec<_>>();
        let mut path = HashSet::with_capacity(words.len());
        let solve = |possible: &[usize], path: &HashSet<usize>| -> usize {
            let mut ret = 0;
            let mut minsize = possible.len();
            for i in 0..words.len() {
                if !path.contains(&i) {
                    let mut groups = vec![Vec::new(); 6];
                    for &j in possible.iter() {
                        if i != j {
                            groups[h[i][j]].push(j);
                        }
                    }
                    if let Some(maxsize) = groups.iter().map(|g| g.len()).max() {
                        if maxsize < minsize {
                            minsize = maxsize;
                            ret = i;
                        }
                    }
                }
            }
            ret
        };
        while !possible.is_empty() {
            let guess = solve(&possible, &path);
            let m = master.guess(words[guess].clone()) as usize;
            if m == 6 {
                return;
            }
            possible.retain(|&i| h[guess][i] == m);
            path.insert(guess);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut master = Master::new(String::from("acckzz"));
        Solution::find_secret_word(
            vec![
                String::from("acckzz"),
                String::from("ccbazz"),
                String::from("eiowzz"),
                String::from("abcczz"),
            ],
            &mut master,
        );
        assert!(master.num_calls <= 10);
    }
}
