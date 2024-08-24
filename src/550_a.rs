use std::{io, process::exit};

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let s: Vec<char> = s.chars().collect();

    let mut ab = None;
    let mut ba = None;

    for (i, pair) in s.windows(2).enumerate() {
        match pair {
            ['A', 'B'] => {
                if ab.is_none() {
                    ab = Some(i);
                }
                if let Some(ba_pos) = ba {
                    if ba_pos + 1 < i {
                        println!("YES");
                        exit(0);
                    }
                }
            }
            ['B', 'A'] => {
                if ba.is_none() {
                    ba = Some(i);
                }
                if let Some(ab_pos) = ab {
                    if ab_pos + 1 < i {
                        println!("YES");
                        exit(0);
                    }
                }
            }
            _ => (),
        }
    }
    println!("NO");
}
