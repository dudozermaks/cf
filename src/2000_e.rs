use std::{collections::BinaryHeap, io};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u16 = buf.trim().parse().unwrap();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, m, k) = {
            let mut buf = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap());

            (
                buf.next().unwrap(),
                buf.next().unwrap(),
                buf.next().unwrap(),
            )
        };

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let _w: u32 = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let mut a: BinaryHeap<u32> = buf
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let mut map: Vec<u32> = vec![0; (n * m) as usize];
        for x in 0..n - k + 1 {
            for i in 0..k {
                map[x as usize + i as usize] += 1;
            }
        }

        for y in 0..m - k + 1 {
            for i in 0..k {
                map[(y as usize + i as usize) * n as usize] += 1;
            }
        }

        // Because we increment it twice, both in x and y loops
        map[0] = 1;

        for y in 1..m {
            for x in 1..n {
                let index = x + (y * n);
                map[index as usize] = map[x as usize] * map[(y * n) as usize];
            }
        }

        map.sort();

        let mut sum = 0;
        for i in map.into_iter().rev() {
            if let Some(gorilla) = a.pop() {
                sum += i as u64 * gorilla as u64
            } else {
                break;
            }
        }

        println!("{sum}");
    }
}
