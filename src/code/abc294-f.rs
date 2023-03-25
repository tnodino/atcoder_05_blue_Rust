// https://atcoder.jp/contests/abc294/tasks/abc294_f

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        K: usize,
    }
    let mut A = Vec::new();
    let mut B = Vec::new();
    for _ in 0..N {
        input! {
            a: f64,
            b: f64,
        }
        A.push(a);
        B.push(b);
    }
    let mut C = Vec::new();
    let mut D = Vec::new();
    for _ in 0..M {
        input! {
            c: f64,
            d: f64,
        }
        C.push(c);
        D.push(d);
    }
    let mut ok = 0.;
    let mut ng = 1.;
    for _ in 0..100 {
        let x = (ok + ng) / 2.;
        let mut v = Vec::new();
        for i in 0..N {
            v.push(A[i] - (B[i] * x / (1. - x)));
        }
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut cnt = 0;
        for i in 0..M {
            let w = C[i] - (D[i] * x / (1. - x));
            let idx = match v.binary_search_by(|x| x.partial_cmp(&-w).unwrap()) {
                Ok(v) => v,
                Err(v) => v,
            };
            cnt +=  N - idx;
        }
        if K <= cnt {
            ok = x;
        }
        else {
            ng = x;
        }
    }
    println!("{}", ok * 100.);
}