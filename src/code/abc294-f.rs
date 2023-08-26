// https://atcoder.jp/contests/abc294/tasks/abc294_f

use proconio::input;
use proconio::fastout;

use std::cmp::Ordering;
fn bisect_left<T: PartialOrd>(vec: &[T], v: &T) -> usize {
    vec.binary_search_by(|x| {
        if x < v {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }).unwrap_or_else(|x| x)
}

#[allow(non_snake_case)]
fn f(x: f64, K: usize, N: usize, A: &Vec<f64>, B: &Vec<f64>, M: usize, C: &Vec<f64>, D: &Vec<f64>) -> bool {
    let mut vec = Vec::new();
    for i in 0..N {
        vec.push(A[i] - (B[i] * x / (1. - x)));
    }
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut cnt = 0;
    for i in 0..M {
        let w = C[i] - (D[i] * x / (1. - x));
        let idx = bisect_left(&vec, &-w);
        cnt +=  N - idx;
    }
    if cnt >= K {
        return true;
    }
    return false;
}

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
        if f(x, K, N, &A, &B, M, &C, &D) {
            ok = x;
        }
        else {
            ng = x;
        }
    }
    println!("{}", ok * 100.);
}