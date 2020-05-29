pub struct Solution {}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<(usize, Vec<usize>)> = vec![(0, vec![]); num_courses as usize];
        for p in prerequisites.iter() {
            graph[p[0] as usize].1.push(p[1] as usize);
            graph[p[1] as usize].0 += 1;
        }
        let mut stack: Vec<usize> = (0..graph.len()).filter(|&i| graph[i].0 == 0).collect();
        let mut count = 0;
        while let Some(last) = stack.pop() {
            count += 1;
            for i in graph[last].1.clone() {
                graph[i].0 -= 1;
                if graph[i].0 == 0 {
                    stack.push(i);
                }
            }
        }
        count == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(true, Solution::can_finish(2, vec![vec![1, 0]]));
    }

    #[test]
    fn example_2() {
        assert_eq!(false, Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
