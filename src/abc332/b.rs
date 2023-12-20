use proconio::input;

#[allow(dead_code)]
pub fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    }

    let mut m_ml = 0;
    let mut g_ml = 0;

    for _ in 0..k {
        if g_ml == g {
            g_ml = 0;
        } else if m_ml == 0 {
            m_ml = m;
        } else {
            let remaining = g - g_ml;
            if remaining > m_ml {
                g_ml = g_ml + m_ml;
                m_ml = 0;
            } else {
                g_ml = g;
                m_ml -= remaining;
            }
        }
    }
    println!("{g_ml} {m_ml}");
}