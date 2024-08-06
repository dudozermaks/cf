use std::{collections::VecDeque, io};

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let t = buf
        .split_whitespace()
        .nth(1)
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let mut max = 0;

    let mut sum = 0;
    let mut queue = VecDeque::new();

    for book in buf.split_whitespace() {
        let book = book.trim().parse::<u32>().unwrap();

        while sum + book as u64 > t {
            if let Some(front) = queue.pop_front() {
                sum -= front as u64;
            } else {
                break;
            }
        }

        if sum + book as u64 <= t {
            queue.push_back(book);
            sum += book as u64;

            if queue.len() > max {
                max = queue.len();
            }
        }
    }

    println!("{}", max);
}
