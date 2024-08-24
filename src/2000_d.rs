use std::io;
use std::io::Write;

fn find_l_and_r<'a>(chars: &Vec<char>, from: usize, to: usize) -> (Option<usize>, Option<usize>) {
    if from >= chars.len() {
        return (None, None);
    }

    let mut first_l = None;
    let mut last_r = None;

    for i in from..to {
        if chars[i] == 'L' {
            first_l = Some(i);
            break;
        }
    }
    for i in (from..to).rev() {
        if chars[i] == 'R' {
            last_r = Some(i);
            break;
        }
    }

    return (first_l, last_r);
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u32 = buf.trim().parse().unwrap();

    let mut lock = io::stdout().lock();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let n: u32 = buf.trim().parse().unwrap();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let a: Vec<u32> = buf
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let mut sum: u64 = 0;

        let chars: Vec<char> = buf.chars().collect();
        let mut last_sum = None;
        let mut prev = None;
        let (mut first_l, mut last_r) = find_l_and_r(&chars, 0, n as usize);

        while first_l.is_some() && last_r.is_some() && first_l.unwrap() < last_r.unwrap() {
            if last_sum == None {
                for i in first_l.unwrap()..=last_r.unwrap() {
                    sum += a[i] as u64;
                }

                last_sum = Some(sum);
            } else {
                let prev: (usize, usize) = prev.unwrap();

                let mut new_sum = last_sum.unwrap();
                for i in prev.0..first_l.unwrap() {
                    new_sum -= a[i] as u64;
                }
                for i in (last_r.unwrap() + 1)..=prev.1 {
                    new_sum -= a[i] as u64;
                }

                sum += new_sum;
                last_sum = Some(new_sum);
            }
            prev = Some((first_l.unwrap(), last_r.unwrap()));

            (first_l, last_r) = find_l_and_r(&chars, first_l.unwrap() + 1, last_r.unwrap());
        }

        writeln!(lock, "{sum}").unwrap();
    }
}
