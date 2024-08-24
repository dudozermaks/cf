use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u32 = buf.trim().parse().unwrap();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: u32 = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a_vec : Vec<u32> = buf.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

        let mut to_occupy_right = None;
        let mut to_occupy_left = None;
        let mut first = None;
        let mut followed_rules = true;

        for a in a_vec {
            if first.is_none() {
                first = Some(a);
                if a > 1 {
                    to_occupy_left = Some(a - 1);
                }
                if a < n {
                    to_occupy_right = Some(a + 1);
                }
            } else {
                if to_occupy_right.is_some_and(|o| o == a) {
                    if to_occupy_right.unwrap() != n {
                        to_occupy_right = Some(to_occupy_right.unwrap() + 1);
                    } else {
                        to_occupy_right = None;
                    }
                } else if to_occupy_left.is_some_and(|o| o == a) {
                    if to_occupy_left.unwrap() != 1 {
                        to_occupy_left = Some(to_occupy_left.unwrap() - 1);
                    } else {
                        to_occupy_left = None;
                    }
                } else {
                    followed_rules = false;
                    break;
                }
            }
        }
        if followed_rules {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}
