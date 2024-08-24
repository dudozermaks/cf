use std::io::Write;
use std::{collections::BinaryHeap, io};

#[derive(Clone, Copy)]
struct Connection {
    destination: usize,
    cost: u64,
}

type Graph = Vec<Vec<Connection>>;

#[derive(Clone, Copy)]
struct Record {
    time: u64,
    prev: Option<usize>,
}

#[derive(PartialEq, Eq)]
struct State {
    min_time: u64,
    node: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.min_time.cmp(&self.min_time)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(graph: Graph) {
    let mut records = vec![None; graph.len()];
    records[0] = Some(Record {
        time: 0,
        prev: None,
    });

    let mut queue = BinaryHeap::new();
    queue.push(State {
        min_time: 0,
        node: 0,
    });

    while let Some(State { min_time, node }) = queue.pop() {
        if records[node].is_some_and(|record| record.time < min_time) {
            continue;
        }

        for connection in &graph[node] {
            let time_will_be = min_time + connection.cost;
            let record = records[connection.destination];

            if record.is_some_and(|r| r.time > time_will_be) || record.is_none() {
                records[connection.destination] = Some(Record {
                    time: time_will_be,
                    prev: Some(node),
                });

                queue.push(State {
                    min_time: time_will_be,
                    node: connection.destination,
                });
            }
        }
    }

    let last_record = records.last().unwrap();

    if last_record.is_none() {
        println!("-1");
    } else {
        let mut path = vec![];
        let mut current = last_record.unwrap().prev;

        while let Some(c) = current {
            path.push(c);
            current = records[c].unwrap().prev;
        }

        let mut lock = io::stdout().lock();
        for node in path.iter().rev() {
            write!(lock, "{} ", node + 1).unwrap();
        }
        writeln!(lock, "{}", graph.len()).unwrap();
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, m) = {
        let mut buf = buf
            .split_whitespace()
            .map(|x| x.trim().parse::<u32>().unwrap());

        (buf.next().unwrap(), buf.next().unwrap())
    };

    let mut graph: Graph = vec![vec![]; n as usize];

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (a, b, w) = {
            let mut buf = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap());

            (
                buf.next().unwrap() - 1,
                buf.next().unwrap() - 1,
                buf.next().unwrap(),
            )
        };

        graph[a].push(Connection {
            destination: b,
            cost: w as u64,
        });
        graph[b].push(Connection {
            destination: a,
            cost: w as u64,
        });
    }

    solve(graph);
}
