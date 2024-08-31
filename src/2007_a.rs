use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (l, r) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<u64>().unwrap(),
                buf.next().unwrap().parse::<u64>().unwrap(),
            )
        };

        let mut even = 0;
        let mut odd = 0;

        for i in l..=r {
            if i % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }

        println!("{}", even.min(odd / 2));
    }
}
