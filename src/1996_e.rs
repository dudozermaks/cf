use std::{collections::HashMap, io};

const M: usize = 1000000007;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let s: Vec<i64> = buf
            .trim()
            .chars()
            .map(|x| if x == '1' { 1 } else { -1 })
            .collect();
        let n = s.len();

        let mut p = vec![0; n + 1];
        for i in 0..n {
            p[i + 1] = s[i] + p[i];
        }

        let mut sums = HashMap::new();
        let mut sum = 0;
        let mut ans = 0;
        for i in 0..(n + 1) {
            if i != 0 {
                sum += s[i - 1];
            }

            let front_score = *sums.get(&sum).unwrap_or(&0);
            let back_score = n - i + 1;
            ans += (front_score * back_score) % M;
            ans %= M;

            if let Some(front_score) = sums.get_mut(&sum) {
                *front_score += i + 1;
            } else {
                sums.insert(sum, i + 1);
            }
        }

        println!("{ans}");
    }
}
