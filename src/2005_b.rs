use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    let mut lock = io::stdout().lock();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, _m, _q) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<u64>().unwrap(),
                buf.next().unwrap().parse::<u64>().unwrap(),
                buf.next().unwrap().parse::<u64>().unwrap(),
            )
        };

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let mut b: Vec<u64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        b.sort();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a: Vec<u64> = buf
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let mut a_s = a.clone();
        a_s.sort();

        let mut bp = 0;
        let mut ap = 0;
        let mut map: HashMap<u64, (Option<u64>, Option<u64>)> = HashMap::new();
        let mut before: Option<u64> = None;

        while bp < b.len() && ap < a.len() {
            let t = b[bp];
            let cur = a_s[ap];

            if t < cur {
                before = Some(t);
                bp += 1;
            } else {
                map.insert(cur, (before, Some(t)));
                ap += 1;
            }
        }
        
        for a in a_s.iter().skip(ap) {
            map.insert(*a, (b.last().copied(), None));
        }

        for aq in a {
            let (before, after) = map[&aq];

            if before.is_some() && after.is_some() {
                let before = before.unwrap();
                let after = after.unwrap();

                let mid = (before + after) / 2;

                writeln!(&mut lock, "{}", mid - before).unwrap();
            } else if before.is_some() {
                writeln!(&mut lock, "{}", n - before.unwrap()).unwrap();
            } else {
                writeln!(&mut lock, "{}", after.unwrap() - 1).unwrap();
            }
        }
    }
}
