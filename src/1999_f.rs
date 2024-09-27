use std::io;
const MODULO: u64 = 1000000000 + 7;

fn neg(mut a: u64) -> u64 {
    let mut res = 1;
    let mut b = MODULO - 2;

    while b > 0 {
        if b % 2 == 1 {
            res = (res * a) % MODULO;
        }

        b /= 2;
        a = (a * a) % MODULO;
    }

    res
}

fn choose(n: usize, k: usize, factorials: &Vec<u64>) -> u64 {
    if n < k {
        return 0;
    }

    (factorials[n] * neg((factorials[k] * factorials[n - k]) % MODULO)) % MODULO
}

fn main() {
    let mut factorials = vec![1];

    for i in 1_u64..=(200000) {
        factorials.push((i * factorials[i as usize - 1]) % MODULO);
    }

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u16 = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, k) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a: Vec<bool> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim() == "1")
            .collect();

        let ones = a.iter().filter(|&&x| x).count();
        let zeros = n - ones;
        let mut ans = 0;

        for i in (k / 2 + 1)..=k.min(ones) {
            ans += (choose(ones, i, &factorials) * choose(zeros, k - i, &factorials)) % MODULO;
            ans %= MODULO;
        }

        println!("{ans}");
    }
}
