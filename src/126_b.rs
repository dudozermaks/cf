use std::{collections::VecDeque, io};

fn main() {
    let mut buf = String::new();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let s: Vec<char> = buf.trim().chars().collect();

    if s.len() < 3 {
        println!("Just a legend");
        return;
    }

    let mut p1 = s.len() - 2;
    let mut t1 = VecDeque::from_iter(s[..s.len() - 1].into_iter().cloned());
    let mut t2 = VecDeque::from_iter(s[1..].into_iter().cloned());
    let s_chopped: String = s[1..s.len() - 1].iter().collect();

    while p1 != 0 {
        t1.pop_back();
        t2.pop_front();

        if t1 == t2 {
            let string : String = t1.iter().collect();
            if s_chopped.contains(&string) {
                println!("{}", string);
                return;
            }
        }

        p1 -= 1;
    }



    println!("Just a legend");
}
