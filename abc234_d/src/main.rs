fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut panel = vec![false; n + 1];

    p[..k - 1].iter().for_each(|&x| panel[x] = true);

    let mut answer = 0;
    p[k - 1..].iter().for_each(|&x| {
        panel[x] = true;

        if (x) > answer {
            answer += 1;
            while !panel[answer] {
                answer += 1;
            };
        }

        println!("{}", answer);
    });
}
