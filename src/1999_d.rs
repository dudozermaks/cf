use std::io;

fn solve<T: io::Write>(s: String, t: String, out: &mut T) {
    let mut sub_i = 0;
    let t : Vec<char> = t.chars().collect();
    let mut sub_c = t[sub_i];
    let mut done = false;

    let res: String = s
        .chars()
        .map(|c| {
            let res = if c == '?' { sub_c } else { c };

            if !done && (c == sub_c || c == '?') {
                sub_i += 1;
                if sub_i == t.len() {
                    done = true;
                } else {
                    sub_c = t[sub_i];
                }
            };

            res
        })
        .collect();

    if done {
        writeln!(out, "YES\n{res}").unwrap();
    } else {
        writeln!(out, "NO").unwrap();
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u32 = buf.trim().parse().unwrap();

    let mut out = io::stdout().lock();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let s = buf.clone().trim().to_string();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let t = buf.clone().trim().to_string();

        solve(s, t, &mut out);
    }
}
