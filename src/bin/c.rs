#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    if n > 2 && m > 2 {
        println!("{}", (n - 2) * (m - 2));
    } else if n == 2 || m == 2 {
        println!("0");
    } else if n == 1 && m == 1 {
        println!("1");
    } else {
        println!("{}", n * m - 2);
    }
}
