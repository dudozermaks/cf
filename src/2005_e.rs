use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (l, n, m) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<u64>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let mut a: Vec<u64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<u64>().unwrap() - 1)
            .collect();

        let mut appeared = vec![false; 7];
        for i in 0..a.len() {
            if appeared[a[i] as usize] {
                a = a[0..i].to_vec();
            }
            else {
                appeared[a[i] as usize] = true;
            }
        }
        let a = a;

        let mut b: Vec<Vec<u64>> = vec![];

        for _ in 0..n {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let b_line: Vec<u64> = buf
                .trim()
                .split_whitespace()
                .map(|x| x.trim().parse::<u64>().unwrap() - 1)
                .collect();

            b.push(b_line);
        }

        let dp = vec![vec![vec![false]; m]; n];
        for (y, line) in b.iter().enumerate().rev() {
            for (x, i) in line.iter().enumerate().rev() {
                
            }
        }
    }
}
