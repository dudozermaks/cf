use std::{collections::HashMap, io};

type Memo = HashMap<u8, u32>;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (n, k, d) = {
        let mut buf = buf.split_whitespace().map(|x| x.trim());
        (
            buf.next().unwrap().parse::<u8>().unwrap(),
            buf.next().unwrap().parse::<u8>().unwrap(),
            buf.next().unwrap().parse::<u8>().unwrap(),
        )
    };

    let mut memo_sat: Memo = HashMap::from([(0, 0)]);
    let mut memo_not_sat: Memo = HashMap::from([(0, 1)]);

    for i in 0..=n {
        for j in 1..=k {
            if i + j > n {
                break;
            }

            if j >= d {
                let to_add = memo_sat.get(&i).unwrap_or(&0) + memo_not_sat.get(&i).unwrap_or(&0);
                if let Some(handle) = memo_sat.get_mut(&(i + j)) {
                    *handle += to_add;
                } else {
                    memo_sat.insert(i + j, to_add);
                }

                *memo_sat.get_mut(&(i + j)).unwrap() = memo_sat[&(i + j)] % 1000000007;
            } else {
                let to_add_sat = *memo_sat.get(&i).unwrap_or(&0);
                if let Some(handle) = memo_sat.get_mut(&(i + j)) {
                    *handle += to_add_sat;
                } else {
                    memo_sat.insert(i + j, to_add_sat);
                }

                let to_add_not_sat = *memo_not_sat.get(&i).unwrap_or(&0);
                if let Some(handle) = memo_not_sat.get_mut(&(i + j)) {
                    *handle += to_add_not_sat;
                } else {
                    memo_not_sat.insert(i + j, to_add_not_sat);
                }

                *memo_sat.get_mut(&(i + j)).unwrap() = memo_sat[&(i + j)] % 1000000007;
                *memo_not_sat.get_mut(&(i + j)).unwrap() = memo_not_sat[&(i + j)] % 1000000007;
            }
        }
    }

    println!("{}", memo_sat.get(&n).unwrap_or(&0));
}
