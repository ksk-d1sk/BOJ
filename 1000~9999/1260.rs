// DFS와 BFS

use std::io::{self, Read};
use std::collections::{BTreeSet, VecDeque};
use std::fmt::{Display, Formatter, Error, Write};

struct PrintVec(Vec<usize>);

impl Display for PrintVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut buffer = String::new();
        for i in &self.0 {
            write!(buffer, "{} ", i).unwrap();
        }
        write!(f, "{}", buffer)
    }
}

struct Search(Vec<VecDeque<usize>>);

impl Search {
    fn dfs(&self, v: usize) -> PrintVec {
        let mut link = self.0.clone();
        let mut answer = Vec::new();
        let mut visit = vec![true; link.len()];
        let mut stack = Vec::new();

        stack.push(v - 1);

        while let Some(p) = stack.pop() {
            if visit[p] {
                visit[p] = false;
                answer.push(p + 1);
                while let Some(next) = link[p].pop_back() {
                    if visit[next] {
                        stack.push(next);
                    }
                }
            }
        }

        PrintVec(answer)
    }

    fn bfs(&self, v: usize) -> PrintVec {
        let mut link = self.0.clone();
        let mut answer = Vec::new();
        let mut visit = vec![true; link.len()];
        let mut queue = VecDeque::new();

        queue.push_back(v - 1);
        visit[v - 1] = false;

        while let Some(p) = queue.pop_front() {
            answer.push(p + 1);
            while let Some(next) = link[p].pop_front() {
                if visit[next] {
                    visit[next] = false;
                    queue.push_back(next);
                }
            }
        }

        PrintVec(answer)
    }
}

fn main() {
    let mut input_buffer = String::new();
    io::stdin().read_to_string(&mut input_buffer).unwrap();
    let mut input_buffer = input_buffer
        .split_ascii_whitespace()
        .map(str::parse)
        .flatten();
    let mut input = || input_buffer.next().unwrap();

    let n = input();
    let m = input();
    let v = input();

    let mut link = vec![BTreeSet::new(); n];

    for _ in 0..m {
        let left  = input() - 1;
        let right = input() - 1;
        link[left].insert(right);
        link[right].insert(left);
    }

    let link = link
        .into_iter()
        .map(|e| e.into_iter().collect())
        .collect();
    let search = Search(link);

    println!(
        "{}\n{}",
        search.dfs(v),
        search.bfs(v)
    );
}
