// https://atcoder.jp/contests/arc051/tasks/arc051_c

use proconio::input;
use proconio::fastout;
use std::collections::BinaryHeap;

const MOD: usize = 1_000_000_007;

fn bin_power(mut x: usize, mut k: usize) -> usize {
    let mut ret = 1;
    while k > 0 {
        if k & 1 > 0 {
            ret *= x;
            ret %= MOD;
        }
        x *= x;
        x %= MOD;
        k >>= 1;
    }
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: usize,
        mut B: usize,
        mut a: [usize; N],
    }
    a.sort();
    if A == 1 {
        for i in 0..N {
            println!("{}", a[i]);
        }
        return;
    }
    let mut heap: BinaryHeap<usize> = BinaryHeap::new();
    for i in 0..N {
        let mut vec = Vec::new();
        while B > 0 && !heap.is_empty() {
            let mut v = heap.pop().unwrap();
            if v >= a[i] {
                vec.push(v);
                continue;
            }
            v *= A;
            if v >= a[i] {
                vec.push(v);
            }
            else {
                heap.push(v);
            }
            B -= 1;
        }
        for i in 0..vec.len() {
            heap.push(vec[i]);
        }
        heap.push(a[i]);
    }
    let mut a = heap.into_vec();
    a.sort();
    for i in 0..N {
        a[i] %= MOD;
    }
    let M = B / N;
    let K = B % N;
    for i in 0..N {
        if i < K {
            a[i] *= bin_power(A, M + 1);
        }
        else {
            a[i] *= bin_power(A, M);
        }
        a[i] %= MOD;
    }
    for i in K..N {
        println!("{}", a[i]);
    }
    for i in 0..K {
        println!("{}", a[i]);
    }
}