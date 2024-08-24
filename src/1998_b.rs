use std::io::Write;
use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u16 = buf.trim().parse().unwrap();

    let mut lock = io::stdout().lock();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let p: Vec<u32> = buf.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

        write!(lock, "{}", p[n-1]).unwrap();

        for i in 0..n-1 {
            write!(lock, " {}", p[i]).unwrap();
        }
        write!(lock, "\n").unwrap();
    }
}
