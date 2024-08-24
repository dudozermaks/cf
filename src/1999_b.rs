use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let num: u32 = buf.trim().parse().unwrap();

    for _ in 0..num {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (a1, a2, b1, b2) = {
            let mut cards = buf
                .split_whitespace()
                .map(|c| c.trim().parse::<u8>().unwrap());

            (
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
                cards.next().unwrap(),
            )
        };

        let mut wins = 0;

        if (a1 >= b1 && a2 > b2) || (a1 > b1 && a2 >= b2) {
            wins += 2;
        }

        if (a1 >= b2 && a2 > b1) || (a1 > b2 && a2 >= b1) {
            wins += 2;
        }

        println!("{}", wins);
    }
}
