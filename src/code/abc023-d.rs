// https://atcoder.jp/contests/abc023/tasks/abc023_d

use proconio::input;
use proconio::fastout;

#[allow(non_snake_case)]
fn f(x: usize, N: usize, H: &Vec<usize>, S: &Vec<usize>) -> bool {
    let mut vec = Vec::new();
    for i in 0..N {
        if x < H[i] {
            return false;
        }
        vec.push((x - H[i]) / S[i]);
    }
    vec.sort();
    for i in 0..N {
        if vec[i] < i {
            return false;
        }
    }
    return true;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut H = Vec::new();
    let mut S = Vec::new();
    for _ in 0..N {
        input! {
            h: usize,
            s: usize,
        }
        H.push(h);
        S.push(s);
    }
    let mut ng = 0;
    let mut ok = 1<<60;
    for _ in 0..100 {
        let mid = (ok + ng) / 2;
        if f(mid, N, &H, &S) {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    println!("{}", ok);
}