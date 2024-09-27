use std::io;

fn main() {
    let mut buf = String::new();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let t: Vec<char> = buf.trim().chars().collect();

    if t.len() == 1 || t.len() == 2 {
        println!("NO");
        return;
    }

    let (mut p1, mut p2) = (t.len() - 2, 1);

    while p1 >= t.len() / 2 {
        if t[0..=p1] == t[p2..t.len()] {
            println!("YES\n{}", t[0..=p1].iter().collect::<String>());
            return;
        }

        p1 -= 1;
        p2 += 1;
    }

    println!("NO")
}
