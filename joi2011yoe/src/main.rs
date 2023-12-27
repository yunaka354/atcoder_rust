#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::usize::MAX;

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
        h: usize,
        w: usize,
        n: usize,
        maze: [Chars; h],
    }

    let mut locations = VecDeque::new();
    let mut sy = 0;
    let mut sx = 0;

    for factory in 0..n {
        for i in 0..h {
            for j in 0..w {
                let c = maze[i][j];
                let oc = c.to_digit(10);

                if let Some(d) = oc {
                    if d - 1 == factory as u32 {
                        locations.push_back((i, j));
                    }
                }

                if c == 'S' {
                    sy = i;
                    sx = j;
                }
            }
        }
    }

    let mut ans = 0;
    while let Some((gy, gx)) = locations.pop_front() {
        ans += bfs(&maze, h, w, sy, sx, gy, gx);
        sy = gy;
        sx = gx;
    }

    println!("{}", ans);
}

fn bfs(maze: &Vec<Vec<char>>, h: usize, w:usize, sy: usize, sx: usize, gy: usize, gx: usize) -> usize {
    let mut costs = vec![vec![MAX; w]; h];
    costs[sy][sx] = 0;
    let mut q = VecDeque::new();
    q.push_back((sy, sx));

    while let Some((cy, cx)) = q.pop_front() {
        for (dy, dx) in DIRECTION_4 {
            let ny = cy as isize + dy;
            let nx = cx as isize + dx;
            if ny < 0 || ny >= h as isize|| nx < 0 || nx >= w as isize {
                continue;
            }

            let ny = ny as usize;
            let nx = nx as usize;

            if maze[ny][nx] != 'X' && costs[ny][nx] == MAX {
                costs[ny][nx] = costs[cy][cx] + 1;
                q.push_back((ny, nx));
            }
        }
    }
    costs[gy][gx]
}