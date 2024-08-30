use std::{io, collections::BinaryHeap};

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();
    let (n, k) = {
        let mut buf = buf.split_whitespace().map(|x| x.trim());
        (
            buf.next().unwrap().parse::<usize>().unwrap(),
            buf.next().unwrap().parse::<usize>().unwrap(),
        )
    };

    if n == k {
        println!("0");
    }


    let mut tree = vec![vec![]; n as usize];
    for _ in 1..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (u, v) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        };
        
        tree[u].push(v);
        tree[v].push(u);
    }

    let mut score = BinaryHeap::new();
    
    dfs(&tree, 0, &mut score, 0, 0);

    let mut res = 0;

    for _ in 0..k {
        res += score.pop().unwrap();
    }

    println!("{res}");
}

fn dfs(tree: &Vec<Vec<usize>>, parent: usize, score: &mut BinaryHeap<i64>, node: usize, depth: u32) -> u32 {
    let mut children_count = 0;

    for child in &tree[node] {
        if *child != parent {
            children_count += dfs(tree, node , score, *child, depth + 1) + 1;
        }
    }

    score.push(depth as i64 - children_count as i64);

    children_count
}
