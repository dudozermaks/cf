use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let tests: usize = buf.trim().parse().unwrap();
    for _ in 0..tests {
        buf.clear();
        io::stdin().read_line(&mut buf).unwrap();
        let n: usize = buf.trim().parse().unwrap();
        let mut points1: [Vec<bool>; 2] = [vec![false; n + 1], vec![false; n + 1]];
        let mut points2: [Vec<usize>; 2] = [vec![], vec![]];
        for i in 0..n {
            buf.clear();
            io::stdin().read_line(&mut buf).unwrap();
            let (x, y) = {
                let mut buf = buf.split_whitespace().map(|x| x.trim());
                (
                    buf.next().unwrap().parse::<usize>().unwrap(),
                    buf.next().unwrap().parse::<usize>().unwrap(),
                )
            };

            points1[y][x] = true;
            points2[y].push(x);
        }

        points2[0].sort();
        points2[1].sort();

        let mut res = 0;
        for i in 0..2 {
            for j in 2..n {
                if points1[i][j] && points1[i][j - 2] && points1[(3 - i) % 2][j - 1] {
                    res += 1;
                }
            }
        }
        let mut ptr1 = 0;
        'main: for (ptr0, p) in points2[0].iter().enumerate() {
            while ptr1 < points2[1].len() && points2[1][ptr1] < *p {
               ptr1 += 1; 
            }

            if ptr1 == points2[1].len() { break 'main;}
            if points2[1][ptr1] == *p {
                res += ptr1;
                res += ptr0;
                res += points2[0].len() - ptr0 - 1;
                res += points2[1].len() - ptr1 - 1;
            }
        } 

        println!("{res}");
    }
}
