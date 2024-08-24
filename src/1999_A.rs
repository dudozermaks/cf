use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let num: u8 = buf.trim().parse().unwrap();

    for _ in 0..num {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n: u8 = buf.trim().parse().unwrap();

        println!("{}", n % 10 + n / 10);
    }
}
