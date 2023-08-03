// https://atcoder.jp/contests/abc201/tasks/abc201_e

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const MOD: usize = 1_000_000_007;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut G = vec![Vec::new(); N];
    for _ in 0..N-1 {
        input! {
            u: usize,
            v: usize,
            w: usize,
        }
        G[u-1].push((v-1, w));
        G[v-1].push((u-1, w));
    }
    let mut dist = vec![0; N];
    let mut que = VecDeque::new();
    que.push_back((0, N));
    while !que.is_empty() {
        let (pos, rev) = que.pop_front().unwrap();
        for (nxt, cost) in G[pos].iter() {
            if *nxt == rev {
                continue;
            }
            dist[*nxt] = dist[pos] ^ cost;
            que.push_back((*nxt, pos));
        }
    }
    let mut ans = 0;
    for i in 0..60 {
        let mut cnt = [0, 0];
        for j in 0..N {
            if dist[j] & (1 << i) == 0 {
                cnt[0] += 1;
            }
            else {
                cnt[1] += 1;
            }
        }
        ans += (1 << i) % MOD * cnt[0] * cnt[1];
        ans %= MOD;
    }
    println!("{}", ans);
}