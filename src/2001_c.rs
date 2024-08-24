use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();

        let mut connections: Vec<(usize, usize)> = vec![];
        let mut queue: HashMap<usize, Vec<usize>> = HashMap::new();
        queue.insert(1, (2..=n).collect());

        'main_loop: while !queue.is_empty() {
            let mut new_queue: HashMap<usize, Vec<usize>> = HashMap::new();

            for (main_node, nodes_to_check) in queue {
                for node in nodes_to_check {
                    println!("? {main_node} {node}");
                    io::stdout().flush().unwrap();

                    buf.clear();
                    io::stdin().read_line(&mut buf).unwrap();
                    let ans: usize = buf.trim().parse().unwrap();
                    if ans == main_node {
                        connections.push((main_node, node));
                        if connections.len() == n-1 {
                            break 'main_loop;
                        }
                    } else {
                        if let Some(handle) = new_queue.get_mut(&ans) {
                            handle.push(node)
                        } else {
                            new_queue.insert(ans, vec![node]);
                        }
                    }
                }
            }

            queue = new_queue;
        }

        print!("!");
        for (a, b) in connections {
            print!(" {a} {b}");
        }
        print!("\n");
        io::stdout().flush().unwrap();
    }
}
