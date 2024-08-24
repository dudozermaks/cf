use std::io;
use std::io::Write;

const MODULO: u32 = 1000000007;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (t, k) = {
        let mut buf = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap());
        (buf.next().unwrap(), buf.next().unwrap())
    };

    let mut max = u32::min_value();

    let mut ab = vec![];

    for _ in 0..t {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (a, b) = {
            let mut buf = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap());
            (buf.next().unwrap(), buf.next().unwrap())
        };
        max = max.max(b);

        ab.push((a, b));
    }

    let mut memo: Vec<u32> = (0..=max).map(|_| 0).collect();
    memo[0] = 1;

    for i in 0..max as usize {
        memo[i + 1] = (memo[i + 1] + memo[i]) % MODULO;
        if i + (k as usize) < (max as usize) + 1 {
            memo[i + k as usize] = (memo[i + k as usize] + memo[i]) % MODULO;
        }
    }

    let mut sum_memo: Vec<u32> = (0..=max).map(|_| 0).collect();
    sum_memo[0] = 0;

    for i in 0..max as usize {
        sum_memo[i + 1] = (sum_memo[i] + memo[i + 1]) % MODULO;
    }

    let mut lock = io::stdout().lock();
    for (a, b) in ab {
        let b_s = sum_memo[b as usize];
        let a_s = sum_memo[a as usize - 1];

        let s = if a_s > b_s {
            b_s + MODULO - a_s
        } else {
            b_s - a_s
        } % MODULO;

        writeln!(lock, "{s}").unwrap();
    }
}
