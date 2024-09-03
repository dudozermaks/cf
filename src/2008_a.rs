use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (a, b) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        if a % 2 == 1 {
            println!("NO")
        }
        else {
            if b % 2 == 0 {
                println!("YES");
            }
            else if a >= 2 {
                println!("YES")
            }
            else {
                println!("NO")
            }
        }
    }
}
