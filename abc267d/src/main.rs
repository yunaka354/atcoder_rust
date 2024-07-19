#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{fastout, input, input_interactive, marker::Chars};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};

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

#[derive(Clone, Debug)]
#[allow(dead_code)]
struct Edge {
    to: usize,
    cap: usize,
    rev: usize,
}

#[allow(dead_code)]
struct MaximumFlow {
    size: usize,
    graph: Vec<Vec<Edge>>,
}

#[allow(dead_code)]
impl MaximumFlow {
    // 頂点数nの残余グラフを準備
    fn new(size: usize) -> Self {
        Self {
            size,
            graph: vec![Vec::<Edge>::new(); size],
        }
    }

    // 頂点aからbに向かう、上限cリットル/秒の辺を追加
    fn add_edge(&mut self, a: usize, b: usize, capacity: usize) {
        let current_a = self.graph[a].len();
        let current_b = self.graph[b].len();
        self.graph[a].push(Edge {
            to: b,
            cap: capacity,
            rev: current_b,
        });
        self.graph[b].push(Edge {
            to: a,
            cap: 0,
            rev: current_a,
        });
    }

    // 深さ優先探索(fはスタートからposに到達する過程での"残余グラフの辺の容量"の最小値)
    // 返り値は流したフローの量（流せない場合は0を返す）
    fn dfs(&mut self, pos: usize, goal: usize, f: usize, used: &mut Vec<bool>) -> usize {
        // ゴールに辿り着いたのでfを返す
        if pos == goal {
            return f;
        };
        used[pos] = true;

        // 重複した可変参照によるエラーを回避するために一時的に配列をコピーする
        let copied_edges = self.graph[pos].clone();

        // 探索する
        for (i, edge) in copied_edges.iter().enumerate() {
            // 容量0の辺は使えない
            if edge.cap == 0 {
                continue;
            }

            // 既に行った頂点はスキップ
            if used[edge.to] {
                continue;
            }

            // 目的地までのパスを探す
            let flow = self.dfs(edge.to, goal, min(f, edge.cap), used);

            // フローを流せる場合、残余グラフの容量をflowだけ増減させる
            if flow > 0 {
                self.graph[pos][i].cap -= flow;
                self.graph[edge.to][edge.rev].cap += flow;
                return flow;
            }
        }
        // 全ての辺を探索しても見つからない場合は0を返す
        0
    }

    // startからgoalまでの最大フローの総流量を返す
    fn max_flow(&mut self, start: usize, goal: usize) -> usize {
        let mut total_flow = 0;
        loop {
            let mut used = vec![false; self.size];
            let f = self.dfs(start, goal, std::usize::MAX, &mut used);

            if f == 0 {
                break;
            };
            total_flow += f;
        }
        total_flow
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

// 座標圧縮
#[allow(dead_code)]
fn compress(v: Vec<usize>) -> Vec<usize> {
    let mut v = v
        .into_iter()
        .enumerate()
        .sorted_by_key(|val| val.1) // (元々のindex, 元々の数字)
        .collect_vec();
    let mut now = v[0].1;
    let mut val = 0 as usize; // 圧縮された座標

    for (_index, x) in v.iter_mut() {
        if now != *x {
            now = *x;
            val += 1;
        }
        *x = val; // 座標をアップデートする。
    }

    v.sort(); // 元々のindexでソートして元に戻す。
    v.into_iter().map(|(_index, x)| x).collect_vec() // 圧縮された座標だけをVecにして返す
}

// 素数判定
#[allow(dead_code)]
fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

// 素因数分解を行う関数
#[allow(dead_code)]
fn prime_factors(mut n: usize) -> HashMap<usize, usize> {
    let mut factors = HashMap::new();
    let mut divisor = 2;

    // 2で割れるだけ割る
    while n % divisor == 0 {
        *factors.entry(divisor).or_insert(0) += 1;
        n /= divisor;
    }

    // 3からsqrt(n)までの奇数で割っていく
    divisor = 3;
    while divisor * divisor <= n {
        while n % divisor == 0 {
            *factors.entry(divisor).or_insert(0) += 1;
            n /= divisor;
        }
        divisor += 2;
    }

    // 最後に残った数が1より大きい場合、それは素数
    if n > 1 {
        factors.insert(n, 1);
    }

    factors
}

// フェルマーの小定理に基づく逆元の計算。問題によってMODが変わる場合は変更すること。
#[allow(dead_code)]
fn mod_inverse(a: usize) -> usize {
    mod_exp(a, MOD - 2)
}

// 繰り返し二乗法を用いたべき乗の計算
#[allow(dead_code)]
fn mod_exp(base: usize, exp: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    let mut res = mod_exp(base, exp / 2);
    res = (res * res) % MOD;
    if exp % 2 != 0 {
        res = (res * base) % MOD;
    }
    res
}

// ベクトル
#[derive(Clone, Copy)]
#[allow(dead_code)]
struct V {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
impl V {
    // 外積の計算
    pub fn cross(&self, other: &V) -> isize {
        return self.x * other.y - self.y * other.x;
    }

    // ベクトルの引き算
    pub fn sub(&self, other: &V) -> Self {
        return Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }

    // CCWの計算
    pub fn ccw(&self, other: &V) -> isize {
        let area = self.cross(other);
        if area > 0 {
            return 1; // counter clock wise
        } else if area < 0 {
            return -1; // clock wise
        } else {
            return 0; // collinear
        }
    }
}

#[allow(dead_code)]
fn lower_bound<T: Ord>(arr: &Vec<T>, x: T) -> usize {
    let mut left = -1;
    let mut right = arr.len() as isize;

    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if arr[mid as usize] >= x {
            right = mid;
        } else {
            left = mid;
        }
    }
    right as usize
}

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize; n],
    }

    let mut dp = vec![vec![isize::MIN; m + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..m {
            if dp[i][j] == isize::MIN {
                continue;
            }
            chmax!(dp[i + 1][j], dp[i][j]);
            chmax!(dp[i + 1][j + 1], dp[i][j] + a[i] * (j as isize + 1));
        }
    }
    let mut ans = isize::MIN;
    for i in 0..=n {
        chmax!(ans, dp[i][m]);
    }
    println!("{}", ans);
}
