use std::io;
use std::io::Write;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let k: u8 = buf.trim().parse().unwrap();

    if k % 2 == 0 {
        println!("NO");
        return;
    }
    if k == 1 {
        println!("YES");
        println!("2 1");
        println!("1 2");
        return;
    }

    let mut res: Vec<(u8, u8)> = vec![];

    for shift in [0, k + 2] {
        for i in 2..=k {
            res.push((1 + shift, i + shift));
            res.push((i + shift, k + 1 + shift));
            res.push((i + shift, k + 2 + shift));

            for j in i+2..=k {
                res.push((i + shift, j+shift));
            }
        }

        for i in ((3 + shift)..=(k + shift))
            .collect::<Vec<u8>>()
            .chunks_exact(2)
        {
            res.push((i[0], i[1]));
        }

        res.push((k + 1 + shift, k + 2 + shift));
    }

    res.push((1, k + 3));

    let mut lock = io::stdout().lock();
    writeln!(lock, "YES").unwrap();
    writeln!(lock, "{} {}", 2 * k + 4, res.len()).unwrap();

    for (r1, r2) in res {
        writeln!(lock, "{r1} {r2}").unwrap();
    }
}
