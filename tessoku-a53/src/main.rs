#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min, Reverse};
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};

#[allow(dead_code)]
const MOD: usize = 1_000_000_000 + 7;

#[allow(dead_code)]
const DIRECTION_4: [(isize, isize); 4] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
];

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
    siz: Vec<usize>
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
    let mut numerator = 1;
    let mut denominator = 1;

    for i in 1..=n {
        numerator = (numerator * i) % MOD;
    }

    for i in 1..=r {
        denominator = (denominator * i) % MOD;
    }

    for i in 1..=(n-r) {
        denominator = (denominator * i) % MOD;
    }
    
    numerator * power(denominator, MOD-2) % MOD
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut min_heap = BinaryHeap::new();

    for _ in 0..q {
        input! {
            query_type: usize,
        }
        
        if query_type == 1 {
            input! { price: usize }
            min_heap.push(Reverse(price));
        } else if query_type == 2 {
            let min = min_heap.peek().unwrap();
            let v = min.0;
            println!("{}", v);
        } else {
            min_heap.pop();
        }
    }
}