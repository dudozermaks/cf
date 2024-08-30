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

    let mut graph = vec![vec![]; n];

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (u, v) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        };

        graph[u].push(v);
        graph[v].push(u);
    }

    let mut colors: Vec<Option<bool>> = vec![None; n];

    for i in 0..n {
        if colors[i].is_some() {
            continue;
        }

        if dfs(&graph, &mut colors, i, true) {
            println!("-1");
            return;
        }
    }

    let mut c1 = vec![];
    let mut c2 = vec![];

    for (i, c) in colors.iter().enumerate() {
        if c.unwrap() {
            c1.push(i);
        } else {
            c2.push(i);
        }
    }

    println!("{}", c1.len());
    for c in c1 {
        print!("{} ", c + 1);
    }
    println!("");

    println!("{}", c2.len());
    for c in c2 {
        print!("{} ", c + 1);
    }
    println!("");
}

fn dfs(graph: &Vec<Vec<usize>>, colors: &mut Vec<Option<bool>>, i: usize, color: bool) -> bool {
    colors[i] = Some(color);

    for n in &graph[i] {
        if let Some(n_color) = colors[*n] {
            if n_color == color {
                return true;
            }
            continue;
        }

        if dfs(graph, colors, *n, !color) {
            return true;
        }
    }

    false
}
