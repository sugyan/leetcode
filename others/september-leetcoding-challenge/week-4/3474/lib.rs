use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut hm: HashMap<&str, HashMap<&str, f64>> = HashMap::new();
        for (i, equation) in equations.iter().enumerate() {
            hm.entry(&equation[0])
                .or_insert_with(HashMap::new)
                .insert(&equation[1], values[i]);
            hm.entry(&equation[1])
                .or_insert_with(HashMap::new)
                .insert(&equation[0], 1.0 / values[i]);
        }
        let mut answer = Vec::with_capacity(queries.len());
        for query in queries.iter() {
            let mut v: Vec<String> = Vec::new();
            answer.push(
                if let Some(ret) = Solution::dfs(&hm, &query[0], &query[1], &mut v) {
                    ret
                } else {
                    -1.0
                },
            );
        }
        answer
    }
    fn dfs(
        hm: &HashMap<&str, HashMap<&str, f64>>,
        src: &str,
        dst: &str,
        v: &mut Vec<String>,
    ) -> Option<f64> {
        if let Some(m) = hm.get(src) {
            for e in m.iter() {
                if e.0 == &dst {
                    return Some(*e.1);
                }
                if v.iter().any(|s| s == e.0) {
                    continue;
                }
                v.push(src.to_string());
                if let Some(ret) = Solution::dfs(hm, &e.0, dst, v) {
                    return Some(*e.1 * ret);
                }
                v.pop();
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000],
            Solution::calc_equation(
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")]
                ],
                vec![2.0, 3.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("e")],
                    vec![String::from("a"), String::from("a")],
                    vec![String::from("x"), String::from("x")]
                ]
            )
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            vec![3.75000, 0.40000, 5.00000, 0.20000],
            Solution::calc_equation(
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("c")],
                    vec![String::from("bc"), String::from("cd")]
                ],
                vec![1.5, 2.5, 5.0],
                vec![
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("c"), String::from("b")],
                    vec![String::from("bc"), String::from("cd")],
                    vec![String::from("cd"), String::from("bc")]
                ]
            )
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            vec![0.50000, 2.00000, -1.00000, -1.00000],
            Solution::calc_equation(
                vec![vec![String::from("a"), String::from("b")],],
                vec![0.5],
                vec![
                    vec![String::from("a"), String::from("b")],
                    vec![String::from("b"), String::from("a")],
                    vec![String::from("a"), String::from("c")],
                    vec![String::from("x"), String::from("y")]
                ]
            )
        );
    }
}
