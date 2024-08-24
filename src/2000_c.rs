use std::{collections::HashMap, io};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u32 = buf.trim().parse().unwrap();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: u32 = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a: Vec<i64> = buf
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let m: u32 = buf.trim().parse().unwrap();

        for _ in 0..m {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            buf = buf.trim().to_string();

            if buf.len() != n as usize {
                println!("NO");
                continue;
            }

            let mut char_to_int: HashMap<char, i64> = HashMap::new();
            let mut failed = false;

            for (i, c) in buf.char_indices() {
                let number = a[i];
                if let Some(char_number) = char_to_int.get(&c) {
                    if *char_number != number {
                        failed = true;
                        break;
                    }
                } else {
                    if char_to_int
                        .iter()
                        .any(|(_, &inserted_number)| inserted_number == number)
                    {
                        failed = true;
                        break;
                    }
                    char_to_int.insert(c, number);
                }
            }

            if failed {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}
