#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
const MOD: usize = 1_000_000_000 + 7;

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
        a: [Chars; 10],
    }

    let start = find_start(&a);
    let count = count_land(&a) + 1;
    
    for i in 0..10 {
        for j in 0..10 {
            let mut copy_a = a.clone();
            copy_a[i][j] = 'o';

            if dfs(&copy_a, start, count) {
                println!("YES");
                return;
            }
        }
    }
    println!("NO");
}

fn dfs(maze: &Vec<Vec<char>>, start: (isize, isize), count: usize) -> bool {
    let mut stack = Vec::new();
    stack.push(start);
    let mut visited = vec![vec![false; 10]; 10];
    
    while let Some(node) = stack.pop() {
        let (ny, nx) = node;

        if ny < 0 || ny >= 10 || nx < 0 || nx >= 10 || visited[ny as usize][nx as usize]{
            continue;
        }
        if maze[ny as usize][nx as usize] == 'x' { continue; }
        visited[ny as usize][nx as usize] = true;

        stack.push((ny-1, nx));
        stack.push((ny+1, nx));
        stack.push((ny, nx-1));
        stack.push((ny, nx+1));
    }

    let mut count_visited = 0;
    for i in 0..10 {
        for j in 0..10 {
            if visited[i][j] {
                count_visited += 1;
            }
        }
    }

    if count == count_visited {
        true
    } else {
        false
    }
}

fn find_start(maze: &Vec<Vec<char>>) -> (isize, isize) {
    for i in 0..10 {
        for j in 0..10 {
            if maze[i][j] == 'o' {
                return (i as isize, j as isize)
            }
        }
    }
    
    panic!("not found");
}

fn count_land(maze: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for i in 0..10 {
        for j in 0..10 {
            if maze[i][j] == 'o' {
                count += 1;
            }
        }
    }
    count
}