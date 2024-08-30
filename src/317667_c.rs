use std::{collections::{HashSet, VecDeque}, io};

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

    let mut tree: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let k: Vec<usize> = buf
            .trim()
            .split_whitespace()
            .skip(1)
            .map(|x| x.trim().parse().unwrap())
            .collect();

        if k.len() < 2 {
            continue;
        }

        for w in k.windows(2) {
            let a = w[0] - 1;
            let b = w[1] - 1;

            tree[a].insert(b);
            tree[b].insert(a);
        }
    }

    let mut visited: Vec<bool> = vec![false; n];
    let mut res = vec![0; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }

        let mut component = HashSet::new();
        let mut queue = VecDeque::from([i]);

        while let Some(node) = queue.pop_front() {
            component.insert(node);
            visited[node] = true;

            for child in &tree[node] {
                if !visited[*child] {
                    queue.push_back(*child);
                }
            }
        }

        for node in &component {
            res[*node] = component.len();
        }
    }

    print!("{}", res.first().unwrap());
    for r in res.iter().skip(1) {
        print!(" {r}");
    }
    print!("\n");
}
