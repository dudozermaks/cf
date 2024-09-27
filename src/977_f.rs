use std::{collections::HashMap, io};

fn main() {
    let mut buf = String::new();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let a: Vec<u64> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut cache: HashMap<u64, u64> = HashMap::new();
    let mut max = (0, 0);

    for ai in &a {
        let mut new_value = 1;
        if let Some(prev) = cache.get(&(ai - 1)) {
            new_value += prev;
        }

        if new_value > max.0 {
            max = (new_value, *ai);
        }

        if let Some(current) = cache.get_mut(&ai) {
            *current = (*current).max(new_value);
        } else {
            cache.insert(*ai, new_value);
        }
    }

    let mut indices = vec![];
    let mut to_find = max.1;

    for i in (0..n).rev() {
        if to_find == a[i] {
            indices.push(i + 1);
            to_find -= 1;
        }
    }

    println!("{}", indices.len());
    indices.reverse();
    for i in indices {
        print!("{i} ");
    }
    print!("\n");
}
