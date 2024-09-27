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

        let mut suff: [HashMap<char, u64>; 2] = [
            HashMap::from_iter(('a'..='z').into_iter().map(|c| (c, 0))),
            HashMap::from_iter(('a'..='z').into_iter().map(|c| (c, 0))),
        ];
        for (i, c) in s.iter().enumerate() {
            *suff[i % 2].get_mut(c).unwrap() += 1;
        }

        if n % 2 == 0 {
            let mut res = n as u64;
            for i in 0..2 {
                let mut max = 0;
                for (_, count) in &suff[i] {
                    max = max.max(*count);
                }
                res -= max;
            }

            println!("{res}");
        } else {
            let mut pref: [HashMap<char, u64>; 2] = [
                HashMap::from_iter(('a'..='z').into_iter().map(|c| (c, 0))),
                HashMap::from_iter(('a'..='z').into_iter().map(|c| (c, 0))),
            ];
            let mut res = u64::max_value();
            for (i, c) in s.iter().enumerate() {
                *suff[i % 2].get_mut(c).unwrap() -= 1;

                let mut new_res = n as u64;
                for i in 0..2 {
                    let mut max = 0;
                    for c in 'a'..='z' {
                        max = max.max(pref[i][&c] + suff[i ^ 1][&c]);
                    }
                    new_res -= max;
                }

                res = res.min(new_res);

                *pref[i % 2].get_mut(c).unwrap() += 1;
            }

            println!("{res}");
        }
    }
}
