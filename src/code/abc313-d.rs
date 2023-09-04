// https://atcoder.jp/contests/abc313/tasks/abc313_d

use proconio::input;
use proconio::source::line::LineSource;


#[allow(non_snake_case)]
fn main() {
    let stdin = std::io::stdin();
    let mut source = LineSource::new(stdin.lock());
    input! {
        from &mut source,
        N: usize,
        K: usize,
    }
    let mut res = vec![0; K+1];
    let mut s = 0;
    for i in 0..=K {
        let mut vec = Vec::new();
        for j in i+1..=i+K {
            vec.push((j % (K + 1)) + 1);
        }
        println!("? {}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        input! {
            from &mut source,
            ret: usize,
        }
        res[i] = ret;
        s ^= res[i];
    }
    let mut ans = vec![0; N];
    for i in 0..=K {
        ans[i] = res[i] ^ s;
    }
    let mut s = 0;
    for i in 0..K-1 {
        s ^= ans[i];
    }
    for i in K+1..N {
        let mut vec = (1..K).collect::<Vec<usize>>();
        vec.push(i + 1);
        println!("? {}", vec.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
        input! {
            from &mut source,
            ret: usize,
        }
        ans[i] = ret ^ s;
    }
    println!("! {}", ans.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(" "));
}