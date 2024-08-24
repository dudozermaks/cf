use std::io;

fn main() {
    let mut buf = String::new();

    io::stdin().read_line(&mut buf).unwrap();

    let tests = buf.trim().parse::<u32>().unwrap();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let (l, r) = {
            let mut lr = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap());
            (lr.next().unwrap(), lr.next().unwrap())
        };

        let mut count = 0;
        let mut temp = l;

        while temp > 0 {
            count += 1;
            temp /= 3;
        }

        count *= 2;

        let mut last = l + 1;
        let mut temp = 3;
        let mut power = 1;

        while temp <= r {
            if temp >= last {
                count += (temp - last) * power;

                last = temp;
            }

            temp *= 3;
            power += 1;
        }

        count += (r - last + 1) * power;

        println!("{}", count);
    }
}
