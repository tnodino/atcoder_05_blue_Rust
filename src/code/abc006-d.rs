// https://atcoder.jp/contests/abc006/tasks/abc006_4

use proconio::input;
use proconio::fastout;

const INF: usize = 1<<32;

fn bisect_left<T: Ord>(vec: &[T], v: &T) -> usize {
    return vec.binary_search_by(|x| x.cmp(v)).unwrap_or_else(|x| x);
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let mut LIS = vec![INF; N+1];
    for _ in 0..N {
        input! {
            c: usize,
        }
        let idx = bisect_left(&LIS, &c);
        LIS[idx] = c;
    }
    for i in 0..=N {
        if LIS[i] == INF {
            println!("{}", N - i);
            return;
        }
    }
}