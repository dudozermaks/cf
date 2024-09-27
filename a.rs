use std::io;

fn main() {
    let mut buf = String::new();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let n: u64 = buf.trim().parse().unwrap();

    println!("{}", n / 5);
}
