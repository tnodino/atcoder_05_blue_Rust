// https://atcoder.jp/contests/abc065/tasks/arc076_b

use proconio::input;
use proconio::fastout;

struct UnionFind {
    par: Vec<isize>,
    sz: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let par = vec![-1; n];
        let sz = vec![1; n];
        Self {
            par,
            sz,
        }
    }

    pub fn root(&mut self, mut x: usize) -> usize {
        while self.par[x] != -1 {
            x = self.par[x] as usize;
        }
        return x;
    }

    pub fn unite(&mut self, u: usize, v: usize) {
        let u = self.root(u);
        let v = self.root(v);
        if u == v {
            return;
        }
        if self.sz[u] < self.sz[v] {
            self.par[u] = v as isize;
            self.sz[v] += self.sz[u];
        }
        else {
            self.par[v] = u as isize;
            self.sz[u] += self.sz[v];
        }
    }

    pub fn same(&mut self, u: usize, v: usize) -> bool {
        if self.root(u) == self.root(v) {
            return true;
        }
        return false;
    }
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut vec = Vec::new();
    for i in 0..N {
        input! {
            x: usize,
            y: usize,
        }
        vec.push((x, y, i));
    }
    let mut edge = Vec::new();
    vec.sort_by(|a, b| a.0.cmp(&b.0));
    for i in 0..N-1 {
        let c = vec[i+1].0 - vec[i].0;
        edge.push((c, vec[i].2, vec[i+1].2));
    }
    vec.sort_by(|a, b| a.1.cmp(&b.1));
    for i in 0..N-1 {
        let c = vec[i+1].1 - vec[i].1;
        edge.push((c, vec[i].2, vec[i+1].2));
    }
    edge.sort_by(|a, b| a.0.cmp(&b.0));
    let mut UF = UnionFind::new(N);
    let mut ans = 0;
    for i in 0..edge.len() {
        let u = edge[i].1;
        let v = edge[i].2;
        if !UF.same(u, v) {
            ans += edge[i].0;
            UF.unite(u, v);
        }
    }
    println!("{}", ans);
}