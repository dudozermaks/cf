use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: u64 = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let mut a: Vec<u64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        a.sort();

        if n < 3 {
            println!("-1");
            continue;
        }

        let mid = a[n as usize / 2];
        let sum: u64 = a.iter().sum();

        let coef = 2 * n * mid;
        if coef < sum {
            println!("0");
        } else {
            println!("{}", coef - sum + 1);
        }
    }
}
