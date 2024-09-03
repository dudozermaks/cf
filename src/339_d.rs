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
    let a: Vec<usize> = buf
        .trim()
        .split_whitespace()
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut hash: Vec<Vec<usize>> = Vec::with_capacity(n);
    hash.push(a);

    for i in 0..n {
        let mut new_vec = Vec::with_capacity(hash[i].len() / 2);
        for numbers in hash[i].chunks(2) {
            let res = if i % 2 == 0 {
                numbers[0] | numbers[1]
            } else {
                numbers[0] ^ numbers[1]
            };

            new_vec.push(res);
        }

        hash.push(new_vec)
    }

    for _ in 0..m {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (p, b) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());

            (
                buf.next().unwrap().parse::<usize>().unwrap() - 1,
                buf.next().unwrap().parse::<usize>().unwrap(),
            )
        };

        hash[0][p] = b;

        let mut current_index = p;
        for i in 0..n {
            let neighbour = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            let next = current_index / 2;

            let res = if i % 2 == 0 {
                hash[i][current_index] | hash[i][neighbour]
            } else {
                hash[i][current_index] ^ hash[i][neighbour]
            };

            hash[i+1][next] = res;
            current_index = next;

        }

        println!("{}", hash[n][0]);
    }
}
