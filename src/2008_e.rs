use std::{collections::HashMap, io};

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
        let s: Vec<char> = buf.trim().chars().collect();

        let mut cache_count: [HashMap<char, u64>; 2] = [
            HashMap::from_iter(('a'..'z').map(|x| (x, 0))),
            HashMap::from_iter(('a'..'z').map(|x| (x, 0))),
        ];

        let mut cache_max: Vec<(char, u64)> = vec![(s[0], 1), (s[1], 1)];

        for (i, c) in s.iter().skip(2).enumerate() {
            let handle = cache_count[i % 2].get_mut(c).unwrap();
            *handle += 1;
            //                     i is already i-2, if you think about it
            if *handle > cache_max[i].1 {
                cache_max.push((*c, *handle));
            } else {
                cache_max.push(*cache_max.last().unwrap());
            }
        }

        if n % 2 == 0 {
            println!("{}", n as u64 - cache_max[n - 1].1 - cache_max[n - 2].1);
        } else {
            let chars_to_count = [cache_max[n - 2].0, cache_max[n - 1].0];
            let mut max_char_count = vec![];
            for i in 0..2 {
                max_char_count.push(if s[i] == chars_to_count[i] { 1 } else { 0 });
            }
            for (i, c) in s.iter().skip(2).enumerate() {
                let char_to_count = chars_to_count[i % 2];
                //                        i is already i-2, if you think about it
                let last = max_char_count[i];
                if char_to_count == *c {
                    max_char_count.push(last + 1);
                } else {
                    max_char_count.push(last);
                }
            }

            let 
            for i in 0..n {
            }
        }
    }
}
