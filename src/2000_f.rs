use std::{collections::BinaryHeap, io, mem::swap, cmp::Reverse};

#[derive(PartialEq, Eq, Debug)]
struct Rect {
    min: u16,
    max: u16,
    sum: u16
}

impl Rect {
    fn new(a: u16, b: u16) -> Rect {
        Rect {
            min: a.min(b),
            max: a.max(b),
            sum: a + b
        }
    }
    /// Score left, steps
    fn steps_to_score(&mut self, score: u16) -> (u16, u16) {
        if score >= self.min + self.max {
            (score - self.min - self.max, self.min * self.max)
        } else {
            let mut steps = 0;

            for _ in 0..score {
                steps += self.min;
                self.max -= 1;
                if self.min > self.max {
                    swap(&mut self.min, &mut self.max);
                }
            }

            (0, steps)
        }
    }
}

impl Ord for Rect {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.sum.cmp(&other.sum)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let tests: u16 = buf.trim().parse().unwrap();

    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (n, k) = {
            let mut buf = buf.split_whitespace().map(|x| x.parse::<u16>().unwrap());

            (buf.next().unwrap(), buf.next().unwrap())
        };

        let mut figures = BinaryHeap::new();
        for _ in 0..n {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let (a, b) = {
                let mut buf = buf.split_whitespace().map(|x| x.parse::<u16>().unwrap());

                (buf.next().unwrap(), buf.next().unwrap())
            };

            figures.push(Reverse(Rect::new(a, b)));
        }

        let mut score = k;
        let mut steps = 0;

        while let Some(mut rect) = figures.pop() {
            // println!("{rect:?}");
            let (new_score, new_steps) = rect.0.steps_to_score(score);

            steps += new_steps;
            score = new_score;

            if score == 0 {
                break;
            }
        }

        if score == 0 {
            println!("{steps}");
        } else {
            println!("-1");
        }
    }
}
