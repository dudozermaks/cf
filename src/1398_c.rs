use std::{collections::HashMap, io};

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let _n: usize = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let mut sum = 0;
        let mut index = 0;
        let mut seen: HashMap<i64, usize> = HashMap::from([(1, 1)]);
        let mut ans = 0;

        for d in buf.trim().chars().map(|x| x.to_digit(10).unwrap() as usize) {
            sum += d;

            let diff = sum as i64 - index as i64;

            ans += seen.get(&diff).unwrap_or(&0);

            if let Some(handle) = seen.get_mut(&diff) {
                *handle += 1;
            } else {
                seen.insert(diff, 1);
            }

            index += 1;
        }

        println!("{ans}");
    }
}
