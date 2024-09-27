use std::{cmp::Reverse, collections::BinaryHeap, io};

type Graph = Vec<Vec<(usize, u64)>>;

fn dijkstra(n: usize, graph: &Graph, horses: &Vec<bool>, start: usize) -> Vec<[Option<u64>; 2]> {
    let mut res = vec![[None, None]; n];
    let mut heap = BinaryHeap::from([Reverse((0, start, horses[start]))]);
    if horses[start] {
        res[start][1] = Some(0);
    }
    else {
        res[start][0] = Some(0);
    }

    while let Some(Reverse((cost, pos, is_on_horse))) = heap.pop() {
        let is_on_horse = if is_on_horse || horses[pos] { 1 } else { 0 };

        if res[pos][is_on_horse].map_or(false, |x| x < cost) {
            continue;
        }

        for child in &graph[pos] {
            let rec = res[child.0][is_on_horse];
            let new_rec = cost + child.1 / if is_on_horse == 1 { 2 } else { 1 };

            if rec.map_or(true, |x| x > new_rec) {
                res[child.0][is_on_horse] = Some(new_rec);

                heap.push(Reverse((new_rec, child.0, is_on_horse == 1)))
            }
        }
    }

    res
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

        let man1 = dijkstra(n, &graph, &a, 0);
        let man2 = dijkstra(n, &graph, &a, n - 1);
        let mut res = None;

        let find_min = |o: [Option<u64>; 2]| {
            let mut res = None;
            for i in o {
                if i.is_some_and(|x| res.map_or(true, |res| x < res)) {
                    res = Some(i.unwrap())
                }
            }
            res
        };

        for i in 0..n {
            let w1 = find_min(man1[i]);
            let w2 = find_min(man2[i]);

            if w1.is_some() && w2.is_some() {
                let max = [w1.unwrap(), w2.unwrap()].iter().max().unwrap().clone();

                if res.map_or(true, |res| res > max) {
                    res = Some(max);
                }
            }
        }

        if let Some(res) = res {
            println!("{res}");
        }
        else {
            println!("-1");
        }
    }
}

