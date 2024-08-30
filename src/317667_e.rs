use std::{io, mem::swap};

struct Forest {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    number_of_edges: Vec<usize>,
}

impl Forest {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
            number_of_edges: vec![0; n],
        }
    }

    fn find_parent(&mut self, mut a: usize) -> usize {
        while self.parents[a] != a {
            self.parents[a] = self.parents[self.parents[a]];
            a = self.parents[a];
        }

        a
    }

    fn merge(&mut self, a: usize, b: usize) {
        let mut a = self.find_parent(a);
        let mut b = self.find_parent(b);

        self.number_of_edges[a] += 1;

        if a != b {
            if self.sizes[a] > self.sizes[b] {
                swap(&mut a, &mut b);
            }
            // assume a <= b

            self.parents[a] = b;
            self.sizes[b] += self.sizes[a];
            self.number_of_edges[b] += self.number_of_edges[a];
        }
    }

    fn possible(&self, n: usize) -> bool {
        let label = self.parents[n];
        let size = self.sizes[label];

        self.number_of_edges[label] == size * (size - 1) / 2
    }
}

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

    let mut forest = Forest::new(n);

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (a, b) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
            )
        };

        forest.merge(a, b);
    }

    for n in 0..n {
        if !forest.possible(n) {
            println!("NO");
            return;
        }
    }

    println!("YES");
}
