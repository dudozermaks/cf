use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n: usize = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
    }
}
