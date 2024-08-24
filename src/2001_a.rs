use std::{io, collections::HashMap};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: u8 = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a: Vec<u8> = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<u8>().unwrap())
            .collect();

        let mut number_to_count = HashMap::new();

        for d in a {
            if let Some(handle) = number_to_count.get_mut(&d) {
                *handle += 1;
            }
            else {
                number_to_count.insert(d, 1);
            }
        }

        let max = number_to_count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().1;

        println!("{}", n - max);
    }
}
