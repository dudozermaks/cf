use std::io;

const M: u128 = 1000000007;

fn pw(mut a: u128, mut b: u128) -> u128 {
    let mut res = 1;
    while b > 0 {
        if b % 2 == 1 {
            res = (res * a) % M;
        }
        a = (a * a) % M;
        b /= 2;
    }
    res
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a: Vec<u128> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let mut ans = 0;
        let mut sum = a[0];
        let mut count = 1;
        let mut q = 0;

        for i in 1..n {
            ans += sum * a[i];
            sum += a[i];
            q += count;
            count += 1;
        }

        let inv_q = pw(q, M - 2);

        println!("{}", (ans * inv_q) % M);
    }
}
