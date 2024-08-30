use std::{collections::BinaryHeap, io};

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    let _n: usize = buf.trim().parse().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();

    let mut score = 0;
    let mut count = 0;
    let mut punishments = BinaryHeap::new();

    for a in buf
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse::<i64>().unwrap())
    {
        if a >= 0 {
            score += a as u64;
            count += 1;
        } else {
            let neg_a = (-a) as u64; 
            if score >= neg_a {
                score -= neg_a;
                count += 1;
                punishments.push(neg_a);
            }
            else if punishments.peek().is_some_and(|peek| peek > &neg_a) {
                score += punishments.pop().unwrap();
                score -= neg_a;
                punishments.push(neg_a);
            }
        }
    }

    println!("{count}");
}
