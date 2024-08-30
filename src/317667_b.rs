use std::io;

fn main() {
    let mut buf = String::new();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let (n, m) = {
        let mut buf = buf.split_whitespace().map(|x| x.trim());
        (
            buf.next().unwrap().parse::<usize>().unwrap(),
            buf.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let cost: Vec<u64> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut connections: Vec<Vec<usize>> = vec![vec![]; n];

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (x, y) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        };

        connections[x].push(y);
        connections[y].push(x);
    }

    let mut visited = vec![false; n];
    let mut visited_count = 0;
    let mut res = 0;
    let mut i = 0_usize;

    while visited_count != n {
        if !visited[i] {
            res += dfs(&connections, &cost, &mut visited, &mut visited_count, i);
        }

        i += 1;
    }

    println!("{res}");
}

fn dfs(
    connections: &Vec<Vec<usize>>,
    cost: &Vec<u64>,
    visited: &mut Vec<bool>,
    visited_count: &mut usize,
    current: usize,
) -> u64 {
    visited[current] = true;
    *visited_count += 1;
    let mut min_cost = cost[current];

    for child in &connections[current] {
        if !visited[*child] {
            min_cost = min_cost.min(dfs(connections, cost, visited, visited_count, *child));
        }
    }

    min_cost
}
