// https://atcoder.jp/contests/abc008/tasks/abc008_3

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        C: [usize; N],
    }
    let mut ans = 0.;
    for i in 0..N {
        let mut cnt = 0;
        for j in 0..N {
            if i == j {
                continue;
            }
            if C[i] % C[j] == 0 {
                cnt += 1;
            }
        }
        if cnt % 2 == 0 {
            let x = cnt as f64;
            ans += (x + 2.) / (x * 2. + 2.);
        }
        else {
            ans += 1. / 2.;
        }
    }
    println!("{}", ans);
}