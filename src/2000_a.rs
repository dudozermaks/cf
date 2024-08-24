use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: u32 = buf.trim().parse().unwrap();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();

        let a: u32 = buf.trim().parse().unwrap();

        let digit_count = (a as f64).log10().floor() as u32 + 1;

        if digit_count < 3 {
            println!("NO")
        } else {
            let pow = (10_i32.pow(digit_count - 2)) as u32;
            let first_two_digits = a / pow;
            let last_digits = a % pow;

            if first_two_digits == 10 && last_digits != 1 && last_digits >= pow / 10 {
                println!("YES");
            } else {
                println!("NO")
            }
        }
    }
}
