use std::{cmp::Reverse, collections::BinaryHeap, io};

#[derive(PartialEq, Eq)]
struct Record {
    weight: u64,
    pos: usize,
    h: bool,
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    n: usize,
    graph: &Vec<Vec<(usize, u64)>>,
    h: &Vec<bool>,
    s: usize,
) -> [Vec<Option<u64>>; 2] {
    let mut res = [vec![None; n], vec![None; n]];
    let mut heap = BinaryHeap::from([Reverse(Record {
        weight: 0,
        pos: s,
        h: h[s],
    })]);

    if h[s] {
        res[1][s] = Some(0);
    }
    else {
        res[0][s] = Some(0);
    }

    while let Some(Reverse(v)) = heap.pop() {
        let on_horse = if v.h || h[v.pos] { 1 } else { 0 };

        if res[on_horse][v.pos].map_or(false, |x| x < v.weight) {
            continue;
        }

        for child in &graph[v.pos] {
            let r = res[on_horse][child.0];
            let new_w = v.weight + child.1 / if on_horse == 1 { 2 } else { 1 };
            if r.is_none() || r.is_some_and(|x| x > new_w) {
                res[on_horse][child.0] = Some(new_w);

                heap.push(Reverse(Record {
                    weight: new_w,
                    pos: child.0,
                    h: on_horse == 1,
                }));
            }
        }
    }

    res
}

fn min(o1: Option<u64>, o2: Option<u64>) -> Option<u64> {
    match (o1, o2) {
        (None, None) => None,
        (None, Some(y)) => Some(y),
        (Some(x), None) => Some(x),
        (Some(x), Some(y)) => Some(x.min(y)),
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, m, h) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };
        let a = {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let a: Vec<usize> = buf
                .trim()
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap() - 1)
                .collect();
            let mut new_a = vec![false; n];
            for i in a {
                new_a[i] = true;
            }

            new_a
        };

        let mut graph = vec![vec![]; n];
        for _ in 0..m {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let (u, v, w) = {
                let mut buf = buf.split_whitespace().map(|x| x.trim());
                (
                    buf.next().unwrap().parse::<usize>().unwrap() - 1,
                    buf.next().unwrap().parse::<usize>().unwrap() - 1,
                    buf.next().unwrap().parse::<u64>().unwrap(),
                )
            };

            graph[u].push((v, w));
            graph[v].push((u, w));
        }

        let d1 = dijkstra(n, &graph, &a, 0);
        let d2 = dijkstra(n, &graph, &a, n - 1);
        let mut res = None;
        for i in 0..n {
            let w1 = min(d1[0][i], d1[1][i]);
            let w2 = min(d2[0][i], d2[1][i]);

            if w1.is_some() && w2.is_some() {
                let w1 = w1.unwrap();
                let w2 = w2.unwrap();
                let score = w1.max(w2);

                if res.is_none() || res.is_some_and(|x| x > score) {
                    res = Some(score);
                }
            }
        }
        if let Some(res) = res {
            println!("{res}")
        } else {
            println!("-1");
        }
    }
}
