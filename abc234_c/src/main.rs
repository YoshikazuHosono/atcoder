use proconio::input;

fn main() {
    input! {
        K: u64,
    }

    println!("{}", format!("{:b}", K).replace("1", "2"));
}
