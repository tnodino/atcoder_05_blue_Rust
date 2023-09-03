// https://atcoder.jp/contests/abc188/tasks/abc188_f

use proconio::input;
use proconio::fastout;
use std::cmp::min;
use std::collections::HashMap;

fn f(x: isize, y: isize, map: &mut HashMap<isize, isize>) -> isize {
    if map.contains_key(&y) {
        return map[&y];
    }
    let ret;
    if y == 1 {
        ret = (x - y).abs();
    }
    else if y % 2 == 0 {
        ret = min((x - y).abs(), f(x, y / 2, map) + 1);
    }
    else {
        ret = min((x - y).abs(), min(f(x, (y + 1) / 2, map), f(x, (y - 1) / 2, map)) + 2);
    }
    map.insert(y, ret);
    return ret;
}

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        X: isize,
        Y: isize,
    }
    let mut map = HashMap::new();
    println!("{}", f(X, Y, &mut map));
}