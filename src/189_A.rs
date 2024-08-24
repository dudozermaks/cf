use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashSet},
    io,
};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, a, b, c) = {
        let mut buf = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<usize>().unwrap());

        (
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
            buf.next().unwrap(),
        )
    };

    let lengths : HashSet<usize> = HashSet::from_iter([a, b, c].into_iter().filter(|&x| x <= n));
    let mut memo = vec![0_u32; n + 1];

    for l in &lengths {
        memo[*l] = 1;
    }

    for i in 0..(n + 1) {
        let max = memo[i];
        if max == 0 {
            continue;
        }

        for l in &lengths {
            let new_pos = i + l;

            if new_pos < n + 1 {
                memo[new_pos] = memo[new_pos].max(max + 1);
            }
        }
    }

    println!("{}", memo.last().unwrap());
}
