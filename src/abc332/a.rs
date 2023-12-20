use proconio::input;

pub fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
        mut pq: [(usize, usize); n],
    }
    let mut sum = 0;
    for (price, qty) in pq {
        sum += price * qty;
    }

    if s > sum {
        sum += k;
    } 

    println!("{sum}");
}