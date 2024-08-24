use std::io;

fn find_all_i<'a, T: Iterator<Item = &'a i64>>(iter: T, onethird: i128) -> Vec<usize> {
    let mut res = vec![];
    let mut sum = 0;

    for (i, n) in iter.enumerate() {
        sum += n;
        if sum as i128 == onethird {
            res.push(i);
        }
    }

    res
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n: usize = buf.trim().parse().unwrap();

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let mut sum: i128 = 0;
    let a: Vec<i64> = buf
        .split_whitespace()
        .map(|x| {
            let x = x.trim().parse().unwrap();
            sum += x as i128;
            x
        })
        .collect();

    if sum % 3 != 0 {
        println!("0");
        return;
    }

    let onethird = sum / 3;
    let all_i = find_all_i(a.iter(), onethird);

    let all_j: Vec<usize> = find_all_i(a.iter().rev(), onethird)
        .into_iter()
        .rev()
        .map(|x| n - 1 - x)
        .collect();

    let mut count = 0;

    let mut j_index = 0;

    for i in all_i {
        while i + 1 >= all_j[j_index] {
            j_index += 1;
            if j_index >= all_j.len() {
                break;
            }
        }
        if j_index >= all_j.len() {
            break;
        }

        count += all_j.len() - j_index;
    }

    println!("{count}");
}
