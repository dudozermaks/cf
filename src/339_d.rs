use std::io;

fn main() {
    let mut buf = String::new();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let (n, m) = {
        let mut buf = buf.split_whitespace().map(|x| x.trim());
        (
            buf.next().unwrap().parse::<usize>().unwrap(),
            buf.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let a: Vec<usize> = buf
         .trim()
         .split_whitespace()
         .map(|x| x.trim().parse().unwrap())
         .collect();
    
}
