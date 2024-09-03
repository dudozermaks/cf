use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (x, y, k) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        let xk = (x as f64 / k as f64).ceil() as usize;
        let yk = (y as f64 / k as f64).ceil() as usize;
        let max = xk.max(yk);

        println!("{}", max * 2 - if yk < xk { 1 } else { 0 });
    }
}
