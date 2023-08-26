// https://atcoder.jp/contests/abc132/tasks/abc132_e

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const INF: usize = 1<<60;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut G = vec![Vec::new(); N*3];
    for _ in 0..M {
        input! {
            u: usize,
            v: usize,
        }
        let u = u - 1;
        let v = v - 1;
        G[u].push(N+v);
        G[N+u].push(N*2+v);
        G[N*2+u].push(v);
    }
    input! {
        S: usize,
        T: usize,
    }
    let mut dist = vec![INF; N*3];
    dist[S-1] = 0;
    let mut que = VecDeque::new();
    que.push_back(S-1);
    while !que.is_empty() {
        let pos = que.pop_front().unwrap();
        for nxt in G[pos].iter() {
            if dist[pos] + 1 < dist[*nxt] {
                dist[*nxt] = dist[pos] + 1;
                que.push_back(*nxt);
            }
        }
    }
    if dist[T-1] == INF {
        println!("-1");
    }
    else {
        println!("{}", dist[T-1] / 3);
    }
}