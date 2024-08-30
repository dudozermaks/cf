use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (_n, t) = {
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

    let mut pointer = 1;

    while pointer < t {
        pointer += a[pointer - 1];
    }

    if pointer == t {
        println!("YES");
    }
    else {
        println!("NO");
    }
}
