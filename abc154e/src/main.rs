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
        n: Chars,
        non_zero_count: usize,
    }

    // dp[確定した桁数][未満フラグ][0でない数字の数]
    let mut dp = vec![vec![vec![0 as usize; 105]; 2]; 105];
    dp[0][0][0] = 1;

    let l = n.len();

    // 全桁の走査
    for i in 0..l {
        let original_digit = n[i].to_digit(10).unwrap();
        // 未満フラグ
        for j in 0..2 {
            // 0ではない数
            for k in 0..=non_zero_count {
                for d in 0..10 {
                    if original_digit < d && j == 0 { continue; } // 未満フラグが立っていない場合は現在の数字までループ
                    let to_be_less = (d < original_digit || j == 1)  as usize; // 元々フラグが立っている or 今回未満フラグを立てるかどうか
                    
                    // 0の場合
                    if d == 0 {
                        dp[i+1][to_be_less][k] += dp[i][j][k];
                    }
                    // 0じゃない数字を入れる場合
                    else {
                        dp[i+1][to_be_less][k+1] += dp[i][j][k];
                    }
                }
            }
        }
    }
    
    println!("{}", dp[l][0][non_zero_count] + dp[l][1][non_zero_count]);
}