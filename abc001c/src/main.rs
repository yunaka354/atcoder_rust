use proconio::fastout;
use proconio::input;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input! {
        (mut Deg, mut Dis): (usize, f64),
    }
    Dis = (Dis / 60. * 10.).round() / 10.;
    let wind = if 0. <= Dis && Dis <= 0.2 {
        0
    } else if 0.3 <= Dis && Dis <= 1.5 {
        1
    } else if 1.6 <= Dis && Dis <= 3.3 {
        2
    } else if 3.4 <= Dis && Dis <= 5.4 {
        3
    } else if 5.5 <= Dis && Dis <= 7.9 {
        4
    } else if 8.0 <= Dis && Dis <= 10.7 {
        5
    } else if 10.8 <= Dis && Dis <= 13.8 {
        6
    } else if 13.9 <= Dis && Dis <= 17.1 {
        7
    } else if 17.2 <= Dis && Dis <= 20.7 {
        8
    } else if 20.8 <= Dis && Dis <= 24.4 {
        9
    } else if 24.5 <= Dis && Dis <= 28.4 {
        10
    } else if 28.5 <= Dis && Dis <= 32.6 {
        11
    } else {
        12
    };
    Deg *= 10;
    let dir = if wind == 0 {
        "C"
    } else if 1125 <= Deg && Deg < 3375 {
        "NNE"
    } else if 3375 <= Deg && Deg < 5625 {
        "NE"
    } else if 5625 <= Deg && Deg < 7875 {
        "ENE"
    } else if 7875 <= Deg && Deg < 10125 {
        "E"
    } else if 10125 <= Deg && Deg < 12375 {
        "ESE"
    } else if 12375 <= Deg && Deg < 14625 {
        "SE"
    } else if 14625 <= Deg && Deg < 16875 {
        "SSE"
    } else if 16875 <= Deg && Deg < 19125 {
        "S"
    } else if 19125 <= Deg && Deg < 21375 {
        "SSW"
    } else if 21375 <= Deg && Deg < 23625 {
        "SW"
    } else if 23625 <= Deg && Deg < 25875 {
        "WSW"
    } else if 25875 <= Deg && Deg < 28125 {
        "W"
    } else if 28125 <= Deg && Deg < 30375 {
        "WNW"
    } else if 30375 <= Deg && Deg < 32625 {
        "NW"
    } else if 32625 <= Deg && Deg < 34875 {
        "NNW"
    } else {
        "N"
    };
    println!("{} {}", dir, wind);
}
