#![allow(unused_imports)]
use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::usize::MAX;

#[allow(dead_code)]
const MOD: usize = 1_000_000_000 + 7;

#[allow(dead_code)]
const DIRECTION_4: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[allow(unused_macros)]
macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}
#[allow(unused_macros)]
macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}
#[allow(unused_macros)]
macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}
#[allow(unused_macros)]
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

// Euclidean algorithm for finding the greatest common divisor.
#[allow(dead_code)]
fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

// Function to calculate the sum of each digit
#[allow(dead_code)]
fn find_sum_of_digits(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

#[allow(dead_code)]
fn round_up(dividend: usize, divisor: usize) -> usize {
    (dividend + divisor - 1) / divisor
}

// Function to determine if the given number is a perfect square
#[allow(dead_code)]
fn is_square(n: isize) -> bool {
    if n < 0 {
        return false;
    }
    let root = (n as f64).sqrt() as isize;
    root * root == n
}

#[allow(dead_code)]
// function to convert decimal to a given base
fn convert_to_base(num: usize, base: usize) -> String {
    if base < 2 {
        panic!("Base must be at least 2");
    }

    let mut result = String::new();
    let mut n = num;

    while n > 0 {
        let digit = n % base;
        let char = if digit < 10 {
            (digit as u8 + b'0') as char
        } else {
            (digit as u8 - 10 + b'a') as char
        };
        result.push(char);
        n /= base;
    }

    result.chars().rev().collect()
}

#[allow(dead_code)]
struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    /// generate UnionFind. nodes = number of nodes (0-index)
    fn new(nodes: usize) -> Self {
        Self {
            par: (0..nodes).collect(),
            siz: vec![1; nodes],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}

#[allow(dead_code)]
fn round_integer(value: i64, n: u32) -> usize {
    let factor = 10i64.pow(n);
    let rounded = ((value as f64) / (factor as f64)).round();
    (rounded * (factor as f64)) as usize
}

#[allow(dead_code)]
fn power(a: usize, b: usize) -> usize {
    let mut p = a;
    let mut ans = 1;
    for i in 0..30 {
        let wari = 1 << i;
        if (b / wari) % 2 == 1 {
            ans = (ans * p) % MOD;
        }
        p = (p * p) % MOD;
    }
    ans
}

#[allow(dead_code)]
fn ncr(n: usize, r: usize) -> usize {
    if r > n {
        return 0;
    }
    let mut res = 1;
    for i in 0..r {
        res *= n - i;
        res /= i + 1;
    }
    res
}

#[allow(dead_code)]
struct SegmentTree {
    pub dat: Vec<isize>,
    pub size: usize,
}

#[allow(dead_code)]
impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n {
            size *= 2;
        }
        SegmentTree {
            dat: vec![0; 300000],
            size,
        }
    }

    fn update(&mut self, mut pos: usize, x: isize) {
        pos = pos + self.size - 1;
        self.dat[pos] = x;
        while pos >= 2 {
            pos /= 2;
            self.dat[pos] = max(self.dat[pos * 2], self.dat[pos * 2 + 1]);
        }
    }

    fn query(&self, l: isize, r: isize, a: isize, b: isize, u: usize) -> isize {
        if r <= a || b <= l {
            return std::isize::MIN;
        }
        if l <= a && b <= r {
            return self.dat[u];
        }

        let m = (a + b) / 2;
        let answer_l = self.query(l, r, a, m, u * 2);
        let answer_r = self.query(l, r, m, b, u * 2 + 1);
        return max(answer_l, answer_r);
    }
}

// nの約数を列挙して返す関数　計算量 (√n)
#[allow(dead_code)]
fn calc_divisors(n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut i = 1;
    while i * i <= n {
        // 割り切れる場合
        if n % i == 0 {
            v.push(i);

            // 同じ数字を足さないように注意
            if n / i != i {
                v.push(n / i);
            }
        }
        i += 1;
    }
    // 小さい順に並べる
    v.sort();
    v
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut g = vec![vec![]; n];

    for i in 0..n - 1 {
        input! {a: usize, b: usize, x: Usize1};
        g[i].push((i + 1, a));
        g[i].push((x, b));
    }

    // ある頂点が確定しているか
    let mut fixed = vec![false; n];
    // 現在時点の最短距離
    let mut cur = vec![1_000_000_000_000_000; n];
    // 始点は距離０
    cur[0] = 0;
    // 優先度付きキュー
    let mut q = BinaryHeap::new();

    // 始点を追加
    q.push(Reverse((cur[0], 0)));

    while let Some(Reverse(pair)) = q.pop() {
        let pos = pair.1;
        // 既に確定している頂点ならスキップ
        if fixed[pos] {
            continue;
        }
        // 確定させる
        fixed[pos] = true;

        for i in 0..g[pos].len() {
            let next = g[pos][i].0;
            let cost = g[pos][i].1;

            // 現在時点の距離より早く到達できるなら更新
            if cur[next] > cur[pos] + cost {
                cur[next] = cur[pos] + cost;
                // キューに追加する
                q.push(Reverse((cur[next], next)));
            }
        }
    }
    println!("{}", cur[n - 1]);
}
