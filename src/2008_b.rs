use std::{io, process::exit};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    'main: for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let s: Vec<char> = buf.trim().chars().collect();

        let n_sqrt = (n as f64).sqrt();
        if n_sqrt.floor() == n_sqrt.ceil() {
            let n_sqrt = n_sqrt as usize;

            for (i, c) in s.chunks(n_sqrt).enumerate() {
                if i == 0 || i == n_sqrt - 1 {
                    if c.iter().any(|&x| x == '0') {
                        println!("No");
                        continue 'main;
                    }
                } else {
                    for (j, char) in c.iter().enumerate() {
                        if j == 0 || j == n_sqrt - 1 {
                            if *char == '0' {
                                println!("No");
                                continue 'main;
                            }
                        } else {
                            if *char == '1' {
                                println!("No");
                                continue 'main;
                            }
                        }
                    }
                }
            }
            println!("Yes")
        } else {
            println!("No");
        }
    }
}
