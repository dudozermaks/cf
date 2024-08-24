use std::{io, ops::RangeInclusive};

fn solve(
    c: Vec<u32>,
    range: &RangeInclusive<u32>,
    x: u32,
    min: u32,
    max: u32,
    sum: u32,
    count: u32,
    remove_from: usize,
) -> u32 {
    let mut res = 0;

    if range.contains(&sum) && max as u64 - min as u64 >= x as u64 && count > 1 {
        res += 1;
    }

    for i in remove_from..c.len() {
        let mut c_clone = c.clone();
        let new_element = c_clone.remove(i);

        res += solve(
            c_clone,
            &range,
            x,
            min.min(new_element),
            max.max(new_element),
            sum + new_element,
            count + 1,
            i,
        );
    }

    res
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let (_n, l, r, x) = {
        let mut b = buf.split_whitespace();

        (
            b.next().unwrap().trim().parse::<u8>().unwrap(),
            b.next().unwrap().trim().parse::<u32>().unwrap(),
            b.next().unwrap().trim().parse::<u32>().unwrap(),
            b.next().unwrap().trim().parse::<u32>().unwrap(),
        )
    };

    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    let c: Vec<u32> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!(
        "{}",
        solve(c, &(l..=r), x, u32::max_value(), u32::min_value(), 0, 0, 0)
    );
}
