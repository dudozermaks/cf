use std::io::{self, Write};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u16 = buf.trim().parse().unwrap();

    for _ in 0..tests {
        let mut start = 1;
        let mut end = 1000;

        loop {
            let one_third = (end - start) / 3;
            let x1 = one_third + start;
            let x2 = one_third * 2 + start;

            println!("? {x1} {x2}");
            io::stdout().flush().unwrap();

            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let ans: u64 = buf.trim().parse().unwrap();

            if (x1 + 1) * (x2 + 1) == ans {
                end = x1;
            }
            else if x1 * (x2 + 1) == ans {
                start = x1 + 1;
                end = x2;
            }
            else {
                start = x2 + 1;
            }

            if start == end {
                println!("! {start}");
                break;
            }
        }
    }
}
