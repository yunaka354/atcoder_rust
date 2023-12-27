#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

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

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    let mut ans = vec![vec!['.'; n];n];

    for i in 0..n {
        for j in 0..n {
            // right top corner
            if i == 0 && j == n-1 {
                ans[i+1][j] = a[i][j];
            }
            // right bottom
            else if i == n-1 && j == n-1 {
                ans[i][j-1] = a[i][j];
            }
            // left bottom
            else if i == n-1 && j == 0 {
                ans[i-1][j] = a[i][j];
            }
            // left top
            else if i == 0 && j == 0 {
                ans[i][j+1] = a[i][j];
            }
            // top
            else if i == 0 {
                ans[i][j+1] = a[i][j];
            }
            // right
            else if j == n-1 {
                ans[i+1][j] = a[i][j];
            }
            // bottom
            else if i == n-1 {
                ans[i][j-1] = a[i][j];
            }
            // left
            else if j == 0 {
                ans[i-1][j] = a[i][j];
            } else {
                ans[i][j] = a[i][j];
            }
        }
    }

    for i in ans {
        let i: String = i.iter().collect();
        println!("{}", i);
    }
}