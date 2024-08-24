use std::io;
const MODULO: u64 = 1000000007;

fn b(n: u32, m: u32, f: &Vec<u64>) -> u128 {
    if m == 0 {
        return 1;
    } else if n == m {
        return 1;
    } else if n < m {
        return 0;
    }


    f[n as usize] as u128 / (f[m as usize] * f[n as usize - m as usize]) as u128
}

fn main() {

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u16 = buf.trim().parse().unwrap();
    let mut factorials = vec![1];

    for i in 1_u64..=(200000) {
        factorials.push((i * factorials[i as usize - 1]) % MODULO);
    }

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let k: u32 = buf
            .split_whitespace()
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        let mut ones: u32 = 0;
        let mut zeros: u32 = 0;

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        for c in buf.chars() {
            if c == '1' {
                ones += 1;
            } else if c == '0' {
                zeros += 1;
            }
        }

        let mut res: u64 = 0;
        for i in ((k + 1) / 2)..=k {
            res += (b(ones, i, &factorials) % MODULO as u128 * b(zeros, k - i, &factorials) % MODULO as u128) as u64;
            res = res % MODULO;
        }

        println!("{res}");
    }
}
