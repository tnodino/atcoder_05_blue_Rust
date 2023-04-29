// https://atcoder.jp/contests/code-festival-2014-morning-middle/tasks/code_festival_morning_med_c

#[allow(non_snake_case)]
fn matrix_product<T: Default + Copy + std::ops::AddAssign + std::ops::Mul<Output = T>>
    (a: &[Vec<T>], b: &[Vec<T>]) -> Vec<Vec<T>> {
    let N = a.len();
    let M = a[0].len();
    let K = b[0].len();
    let mut c = vec![vec![T::default(); K]; N];
    for i in 0..N {
        for k in 0..M {
            for j in 0..K {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    return c;
}

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        p: f64,
        n: usize,
    }
    let M = 64;
    let mut mat = vec![vec![vec![1.-p, p], vec![p, 1.-p]]; M];
    for i in 0..M-1 {
        mat[i+1] = matrix_product(&mat[i], &mat[i]);
        mat[i+1][0][1] = 1. - mat[i+1][0][0];
        mat[i+1][1][0] = 1. - mat[i+1][1][1];
    }
    let mut ans = vec![vec![1., 0.], vec![0., 0.]];
    for i in 0..M {
        if n & (1 << i) > 0 {
            ans = matrix_product(&ans, &mat[i]);
        }
    }
    println!("{}", ans[0][1]);
}