use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, d, k) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        let mut ls = vec![0; n];
        let mut rs = vec![0; n];
        for _ in 0..k {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let (l, r) = {
                let mut buf = buf.split_whitespace().map(|x| x.trim());
                (
                    buf.next().unwrap().parse::<usize>().unwrap() - 1,
                    buf.next().unwrap().parse::<usize>().unwrap() - 1,
                )
            };

            ls[l] += 1;
            rs[r] += 1;
        }

        let mut score = 0;
        let mut min = (u32::max_value(), 0);
        let mut max = (0, 0);

        for i in 0..d {
            score += ls[i];
        }

        if score > max.0 {
            max = (score, 0);
        }
        if score < min.0 {
            min = (score, 0);
        }

        for i in 0..(n - d) {
            score += ls[i + d];
            score -= rs[i];

            if score > max.0 {
                max = (score, i + 1);
            }
            if score < min.0 {
                min = (score, i + 1);
            }
        }

        println!("{} {}", max.1 + 1, min.1 + 1)
    }
}
