use std::{io, process::exit};

fn main() {
    let reminder_to_second_digit = [
        (vec![1, 5, 9], vec![3, 7]),
        (vec![2, 6], vec![0, 4, 8]),
        (vec![3, 7], vec![1, 5, 9]),
    ];

    let mut number = String::new();
    io::stdin().read_line(&mut number).unwrap();

    let digits: Vec<u8> = number
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    digits.iter().for_each(|d| match d {
        0 => {
            println!("YES\n0");
            exit(0)
        }
        8 => {
            println!("YES\n8");
            exit(0)
        }
        _ => (),
    });

    for (i, x) in digits.iter().rev().enumerate() {
        let allowed_first = [6, 4, 2];

        if !allowed_first.contains(&x) {
            continue;
        }

        let reminder = allowed_first.iter().position(|f| f == x).unwrap() + 1;
        let (two_digit_number, three_digit_number) = &reminder_to_second_digit[reminder - 1];

        for (j, y) in digits.iter().rev().skip(i + 1).enumerate() {
            if two_digit_number.contains(y) {
                println!("YES\n{}", y * 10 + x);
                exit(0);
            }

            if three_digit_number.contains(y) {
                for z in digits.iter().rev().skip(i + j + 2) {
                    if z % 2 == 1 {
                        println!("YES\n{}", *z as u16 * 100 + *y as u16 * 10 + *x as u16);
                        exit(0);
                    }
                }
            }
        }
    }

    println!("NO")
}
