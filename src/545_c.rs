use std::io;

#[derive(Debug)]
struct Tree {
    pos: usize,
    height: usize,
}

impl Tree {
    fn fell_left(&self, record: &Record) -> Option<Record> {
        if self.pos >= self.height {
            let smallest_pos = self.pos - self.height;
            if smallest_pos > record.last_occupied {
                return Some(Record {
                    count: record.count + 1,
                    last_occupied: self.pos,
                });
            }
        }
        None
    }

    fn fell_right(&self, record: &Record, next_tree: Option<&Tree>) -> Option<Record> {
        if record.last_occupied < self.pos {
            let last_occupied = self.pos + self.height;
            if let Some(tree) = next_tree {
                if tree.pos <= last_occupied {
                    return None;
                }
            }

            return Some(Record {
                count: record.count + 1,
                last_occupied,
            });
        }

        None
    }

    fn fell(&self, r1: &Record, r2: &Record, next_tree: Option<&Tree>) -> Record {
        let mut max: Record = *r1.max(r2);
        max.last_occupied = self.pos;

        for r in [r1, r2] {
            if let Some(record) = self.fell_right(r, next_tree) {
                max = max.max(record);
            }
            if let Some(record) = self.fell_left(r) {
                max = max.max(record);
            }
        }

        max
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Record {
    count: u64,
    last_occupied: usize,
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.count.cmp(&other.count) {
            std::cmp::Ordering::Equal => other.last_occupied.cmp(&self.last_occupied),
            other_order => other_order,
        }
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();

    let mut trees = vec![];

    for _ in 0..n {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let (x, h) = {
            let mut buf = buf
                .split_whitespace()
                .map(|x| x.trim().parse::<usize>().unwrap());

            (buf.next().unwrap(), buf.next().unwrap())
        };

        trees.push(Tree { pos: x, height: h });
    }

    let mut records = vec![
        Record {
            count: 0,
            last_occupied: 0,
        },
        Record {
            count: 1,
            last_occupied: trees[0].pos,
        },
    ];
    let mut tree_index = 1;

    for t in trees.iter().skip(1) {
        let last_record = &records[tree_index];
        let mut last_last_record = records[tree_index - 1];
        last_last_record.last_occupied = trees[tree_index - 1].pos;

        records.push(t.fell(&last_record, &last_last_record, trees.get(tree_index + 1)));

        tree_index += 1;
    }

    println!("{}", records.last().unwrap().count);
}
