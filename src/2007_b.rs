use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (_n, m) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a: Vec<i64> = buf
             .trim()
             .split_whitespace()
             .map(|x| x.trim().parse().unwrap())
             .collect();
        
        let mut max = a[0];
        for i in a.iter() {
            max = max.max(*i);
        }
        
        let mut maxes = vec![];
        for _ in 0..m {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let (c, l, r) = {
                let mut buf = buf.split_whitespace().map(|x| x.trim());
                (
                    buf.next().unwrap() == "+",
                    buf.next().unwrap().parse::<i64>().unwrap(),
                    buf.next().unwrap().parse::<i64>().unwrap(),
                )
            };

            if max >= l && max <= r {
                if c { max += 1;}
                else { max -= 1;}
            }

            maxes.push(max);
        }

        for m in maxes {
            print!("{m} ");
        }
        print!("\n");
    }
}
