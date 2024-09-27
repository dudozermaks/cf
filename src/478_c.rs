use std::{io, mem::swap};

fn sort(a: &mut u64, b: &mut u64, c: &mut u64) {
    if b > a {
        swap(a, b);
    }
    if c > a {
        swap(a, c);
    }
    if c > b {
        swap(b, c);
    }
}

fn steps_to_eq(a: u64, b: u64) -> u64 {
    b.min(a - b)
}

fn about_equal(a: u64, b: u64) -> bool {
    a - b < 2
}

fn main() {
    let mut buf = String::new();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let (mut a, mut b, mut c) = {
        let mut buf = buf.split_whitespace().map(|x| x.trim());
        (
            buf.next().unwrap().parse::<u64>().unwrap(),
            buf.next().unwrap().parse::<u64>().unwrap(),
            buf.next().unwrap().parse::<u64>().unwrap(),
        )
    };

    let mut count = 0;

    sort(&mut a, &mut b, &mut c);

    while !about_equal(a, c) {
        println!("{a}, {b}, {c}");
        if c == 0 {
            
        } else if about_equal(a, b) {
            let x = steps_to_eq(a + b, c);
            a -= x;
            b -= x;
            c -= x;
            count += x;
        } else if about_equal(b, c) {
            let x = steps_to_eq(a, b + c);
            a -= 2 * x;
            b -= x / 2;
            c -= x / 2;
            if x % 2 == 1 {
                b -= 1;
            }
            count += x;
        } else {
            let x = steps_to_eq(a, b);
            a -= 2 * x;
            b -= x;
            count += x;
        }
        sort(&mut a, &mut b, &mut c);
    }

    count += a.min(b).min(c);

    println!("{count}");
}
