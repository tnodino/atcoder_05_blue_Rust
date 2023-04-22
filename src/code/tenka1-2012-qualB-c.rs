// https://atcoder.jp/contests/tenka1-2012-qualB/tasks/tenka1_2012_7

use proconio::input;
use proconio::fastout;
use std::cmp::min;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
    }
    let M = 1440;
    let mut T = Vec::new();
    for _ in 0..N {
        input! {
            Ts: String,
            Te: String,
        }
        let mut Ts = Ts.split(':');
        let hs = Ts.next().unwrap().parse::<usize>().unwrap();
        let ws = Ts.next().unwrap().parse::<usize>().unwrap();
        let s = hs * 60 + ws;
        let mut Te = Te.split(':');
        let he = Te.next().unwrap().parse::<usize>().unwrap();
        let we = Te.next().unwrap().parse::<usize>().unwrap();
        let e = he * 60 + we;
        let mut t = vec![false; M];
        for i in s..e {
            t[i%M] = true;
        }
        T.push(t);
    }
    let mut check = vec![vec![false; N]; N];
    for i in 0..N {
        for j in i+1..N {
            for k in 0..M {
                if T[i][k] & T[j][k] {
                    check[i][j] = true;
                }
            }
        }
    }
    let mut DP = vec![N; 1<<N];
    for bit in 1..1<<N {
        let mut vec = Vec::new();
        for i in 0..N {
            if bit & (1 << i) > 0 {
                vec.push(i);
            }
        }
        let v = vec.len();
        let mut flg = false;
        for i in 0..v {
            for j in i+1..v {
                flg |= check[vec[i]][vec[j]];
            }
        }
        if !flg {
            DP[bit] = 1;
        }
    }
    for bit1 in 1..1<<N {
        for bit2 in 1..1<<N {
            if bit1 & bit2 > 0 {
                continue;
            }
            DP[bit1|bit2] = min(DP[bit1|bit2], DP[bit1] + DP[bit2]);
        }
    }
    println!("{}", DP[(1<<N)-1]);
}