use std::io::Write;
use std::io::{self, stdout};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    let mut lock = stdout().lock();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: u32 = buf.trim().parse().unwrap();

        if n == 1 {
            write!(lock, "1").unwrap();
        }
        else if n % 2 == 0 {
            write!(lock, "-1").unwrap();
        }
        else {
            for i in 1..=n {
                if i % 2 == 1 {
                    write!(lock, "{i} ").unwrap();
                }
            }
            for i in (3..=n).rev() {
                if i % 2 == 0 {
                    write!(lock, "{i} ").unwrap();
                }
            }
            write!(lock, "2").unwrap();
        }
        write!(lock, "\n").unwrap();
    }
}
