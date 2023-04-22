// https://atcoder.jp/contests/code-festival-2016-qualb/tasks/codefestival_2016_qualB_d

use proconio::input;
use proconio::fastout;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        A: [usize; N],
    }
    let mut x = 2;
    let mut ans = A[0] - 1;
    for i in 1..N {
        if A[i] < x {
            continue;
        }
        else if A[i] == x {
            x += 1;
        }
        else {
            ans += (A[i] - 1) / x;
        }
    }
    println!("{}", ans);
}