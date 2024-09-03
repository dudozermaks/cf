use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        let mut mem = vec![];
        for _ in 0..n {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let hash_i = buf.trim().find('#').unwrap() + 1;
            mem.push(hash_i);
        }
        for i in mem.iter().rev() {
            print!("{i} ");
        }
        print!("\n");
    }
}
