use std::{collections::BinaryHeap, io};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for _ in 0..n - 1 {
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
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let mut colors: Vec<Option<bool>> = buf
            .trim()
            .chars()
            .map(|x| if x == '?' { None } else { Some(x == '1') })
            .collect();



        // Answer
        let mut counts = vec![0; n];

        count(
            &graph,
            &colors,
            &mut counts,
            &mut vec![false; n],
            0,
            0,
            colors[0].unwrap(),
        );

        let mut score = 0;
        for i in 1..n {
            if graph[i].len() == 1 {
                if counts[i] != 0 {
                    score += 1;
                }
            }
        }

        println!("{score}");
    }
}

fn count(
    graph: &Vec<Vec<usize>>,
    colors: &Vec<Option<bool>>,
    counts: &mut Vec<i64>,
    visited: &mut Vec<bool>,
    current_node: usize,
    mut score: i64,
    parrent_color: bool,
) {
    visited[current_node] = true;
    let color = colors[current_node].unwrap();

    if parrent_color && !color {
        score += 1;
    } else if !parrent_color && color {
        score -= 1;
    }
    counts[current_node] = score;

    for child in &graph[current_node] {
        if !visited[*child] {
            count(graph, colors, counts, visited, *child, score, color);
        }
    }
}
