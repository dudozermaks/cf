use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, k) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<u64>().unwrap(),
                buf.next().unwrap().parse::<u64>().unwrap(),
            )
        };

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a: Vec<u64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let mut gold = 0;
        let mut count = 0;
        for i in a {
            if i >= k {
                gold += i;
            } else if i == 0 && gold != 0 {
                gold -= 1;
                count += 1;
            }
        }

        println!("{count}");
    }
}
