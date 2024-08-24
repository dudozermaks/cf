use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u8 = buf.trim().parse().unwrap();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (xc, yc, mut k) = {
            let mut buf = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<i64>().unwrap());

            (
                buf.next().unwrap(),
                buf.next().unwrap(),
                buf.next().unwrap(),
            )
        };

        let mut i = 1;
        while k != 0 {
            if k == 1 {
                println!("{} {}", xc, yc);
                k -= 1;
            }
            else {
                println!("{} {}", i + xc, i + yc);
                println!("{} {}", -i + xc, -i + yc);
                i += 1;

                k -= 2;
            }
        }
    }
}
