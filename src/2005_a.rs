use std::io::{self, Write};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();
        let chars = ['a', 'e', 'i', 'o', 'u'];

        let q = n / 5;
        let mut n = n - (q * 5);

        let mut lock = io::stdout().lock();
        for i in 0..5 {
            for _ in 0..q {
                write!(&mut lock, "{}", chars[i]).unwrap()
            }
            if n > 0 {
                n -= 1;
                write!(&mut lock, "{}", chars[i]).unwrap()
            }

        }

        print!("\n");
    }
}
