use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let p: Vec<usize> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap() - 1)
            .collect();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let s: Vec<bool> = buf
            .trim()
            .chars()
            .map(|x| x.to_digit(10).unwrap() == 1)
            .collect();

        let mut cache: Vec<Option<u64>> = vec![None; n];

        for i in 0..n {
            if let Some(handle) = cache[i] {
                print!("{handle} ");
            } else {
                let mut to_cache: Vec<usize> = vec![i];

                let mut count = 0;
                if !s[i] {
                    count += 1;
                }
                
                let mut current_node = p[i];

                while i != current_node {
                    if !s[current_node] {
                        count += 1;
                    }
                    to_cache.push(current_node);

                    current_node = p[current_node];
                }

                for c in to_cache {
                    cache[c] = Some(count);
                }

                print!("{count} ");
            }
        }

        print!("\n");
    }
}
