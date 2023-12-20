use proconio::input;

#[allow(dead_code)]
pub fn main() {
    input! {
        n: usize,
        m: usize,
        s: String,
    }

    let mut current_blank = m;
    let mut current_logo = 0;
    let mut buy = 0;
    for action in s.chars() {
        if action == '0' {
            current_blank = m;
            current_logo = buy;
        }
        if action == '1' {
            if current_blank > 0 {
                current_blank -= 1;
            } else if current_logo > 0{
                current_logo -= 1;
            } else {
                buy += 1;
            }
        }
        if action == '2' {
            if current_logo > 0 {
                current_logo -= 1;
            } else {
                buy += 1;
            }
        }
    }
    println!("{buy}");
}