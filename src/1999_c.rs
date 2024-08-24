use std::{io, process::exit};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u32 = buf.trim().parse().unwrap();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, s, m) = {
            let mut nsm = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<u64>().unwrap());
            (
                nsm.next().unwrap(),
                nsm.next().unwrap(),
                nsm.next().unwrap(),
            )
        };

        let mut last_end = 0;
        let mut yes = false;

        for _ in 0..n {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            if yes {
                continue;
            }

            let (l, r) = {
                let mut lr = buf
                    .split_whitespace()
                    .map(|x| x.trim().parse::<u64>().unwrap());
                (lr.next().unwrap(), lr.next().unwrap())
            };

            if l - last_end >= s {
                yes = true;
            }

            last_end = r;
        }

        if yes || m - last_end >= s {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
