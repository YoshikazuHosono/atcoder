use proconio::input;

fn main() {
    input! {
        t: u32,
    }

    // f(f(f(t)+t)+f(f(t)))
    println!("{}", f(f(f(t) + t) + f(f(t))));
}

fn f(x: u32) -> u32 {
    // f(x)=x2+2x+3
    x.pow(2) + 2 * x + 3
}
