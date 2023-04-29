// https://atcoder.jp/contests/arc003/tasks/arc003_3

use proconio::input;
use proconio::fastout;
use std::collections::VecDeque;

const DX: &[usize] = &[!0, 1, 0, 0];
const DY: &[usize] = &[0, 0, !0, 1];

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
    }
    let mut light = vec![1.];
    for i in 0..N*M {
        light.push(light[i] * 0.99);
    }
    let mut c = Vec::new();
    for _ in 0..N {
        input! {
            s: String,
        }
        let s = s.chars().collect::<Vec<char>>();
        c.push(s);
    }
    let mut sx = 0;
    let mut sy = 0;
    let mut gx = 0;
    let mut gy = 0;
    for i in 0..N {
        for j in 0..M {
            if c[i][j] == 's' {
                sx = i;
                sy = j;
            }
            if c[i][j] == 'g' {
                gx = i;
                gy = j;
            }
        }
    }
    let mut ok = -1e-9;
    let mut ng = 9.;
    while ng - ok > 1e-9 {
        let mid = (ok + ng) / 2.;
        let mut flg = vec![vec![false; M]; N];
        flg[sx][sy] = true;
        let mut deq = VecDeque::new();
        deq.push_back((sx, sy, 0));
        while !deq.is_empty() {
            let (x, y, time) = deq.pop_front().unwrap();
            for d in 0..4 {
                let nx = x.wrapping_add(DX[d]);
                let ny = y.wrapping_add(DY[d]);
                if N <= nx || M <= ny {
                    continue;
                }
                if flg[nx][ny] {
                    continue;
                }
                if c[nx][ny] == 's' || c[nx][ny] == '#' {
                    continue;
                }
                if c[nx][ny] == 'g' {
                    flg[nx][ny] = true;
                    continue;
                }
                let t = ((c[nx][ny] as usize - 48) as f64) * light[time+1];
                if mid <= t {
                    flg[nx][ny] = true;
                    deq.push_back((nx, ny, time + 1));
                }
            }
        }
        if flg[gx][gy] {
            ok = mid;
        }
        else {
            ng = mid;
        }
    }
    if ok == -1e-9 {
        println!("-1");
    }
    else {
        println!("{}", ok);
    }
}