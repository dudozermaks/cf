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
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        let mut count = 0;
        let mut diff = 1;
        let mut s = 0;
        while l + s <= r {
            s += diff;
            diff += 1;
            count += 1;
        }

        println!("{}", count);
    }
}
