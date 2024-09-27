use std::io;

fn sum(from: u64, to: u64) -> u64 {
    (to - from + 1) * (to + from) / 2
}

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

        let from = k;
        let to = from + n - 1;

        let mut p1 = from;
        let mut p2 = to;

        while p1 != p2 - 1 {
            let mid = (p1 + p2) / 2;
            let s1 = sum(from, mid);
            let s2 = sum(mid, to) - mid;

            if s1 < s2 {
                p1 = mid;
            }
            else {
                p2 = mid;
            }
        }

        let mid = (p1 + p2) / 2;
        let s1 = sum(from, mid);
        let s2 = sum(mid, to) - mid;

        let s3 = sum(from, mid) - mid;
        let s4 = sum(mid, to);

        println!("{}", (s1.abs_diff(s2)).min(s3.abs_diff(s4)));
    }
}
