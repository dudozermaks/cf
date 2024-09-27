use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (mut n, k) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<u64>().unwrap(),
                buf.next().unwrap().parse::<u64>().unwrap(),
            )
        };

        if k == 1 {
            if n % 2 == 0 {
                println!("YES");
            } else {
                println!("NO");
            }
            continue;
        }

        let mut first_year = n - k + 1;

        if n % 2 == 1 {
            n += 1;
        }
        if first_year % 2 == 1 {
            first_year -= 1;
        }

        let odd = (n - first_year) / 2;

        if odd % 2 == 0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
