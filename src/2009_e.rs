use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, k) = {
            let mut buf = buf.split_whitespace().map(|x| x.trim());
            (
                buf.next().unwrap().parse::<i64>().unwrap(),
                buf.next().unwrap().parse::<i64>().unwrap(),
            )
        };

        let mut cur_min = k;
        let mut cur_max = k + n - 1;

        let mut sum = 0;
        
        while cur_min < cur_max {
            if sum >= cur_max {
                sum -= cur_max;
                cur_max -= 1;
            } else {
                sum += cur_min;
                cur_min += 1;
            }
        }

        println!("{}", sum.min((sum - cur_max).abs()));
    }
}
