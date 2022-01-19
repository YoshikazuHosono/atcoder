use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: i32,
        a: [[i32; 2]; n],
    }

    let mut answer = 0f64;

    for dot1 in a.iter() {
        for dot2 in a.iter() {
            let tmp = distance(dot1[0], dot1[1], dot2[0], dot2[1]);
            answer = if answer > tmp {
                answer
            } else {
                tmp
            }
        }
    }

    println!("{}", answer);
}

fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f64 {
    (((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64).sqrt()
}
