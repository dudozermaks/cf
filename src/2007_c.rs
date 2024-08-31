use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, a, b) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<u64>().unwrap(),
                buf.next().unwrap().parse::<u64>().unwrap(),
            )
        };

        let g = gcd(a, b);

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let mut c: Vec<u64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<u64>().unwrap() % g)
            .collect();

        c.sort();

        let mut res = c[n - 1] - c[0];

        for i in 1..n {
            res = res.min(c[i - 1] + g - c[i]);
        }

        println!("{res}");
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
