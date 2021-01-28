use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    // let inputs = std::fs::read_to_string("inputs/vestigium").unwrap();
    // let mut it = inputs.split('\n');

    let stdin = std::io::stdin();
    let mut it = stdin.lock().lines();

    let n_cases = it.next().unwrap().unwrap().parse::<usize>().unwrap();
    for i in 0..n_cases {
        let n = it.next().unwrap().unwrap().parse::<usize>().unwrap();
        let mut col_checkers = Vec::<HashSet<u32>>::with_capacity(n);
        col_checkers.resize(n, HashSet::new());
        let mut repeated_rows = HashSet::new();
        let mut repeated_cols = HashSet::new();
        let mut trace = 0;

        for r in 0..n {
            let mut row = it
                .next()
                .unwrap()
                .unwrap()
                .split(' ')
                .flat_map(|str| str.parse::<u32>().ok())
                .collect::<Vec<u32>>();

            trace += row[r];

            row.iter().enumerate().for_each(|(idx, &item)| {
                if col_checkers[idx].contains(&item) {
                    repeated_cols.insert(idx);
                }
                col_checkers[idx].insert(item);
            });

            if row.len() != row.clone().into_iter().collect::<HashSet<u32>>().len() {
                repeated_rows.insert(r);
            }
        }

        println!("Case #{}: {} {} {}", i + 1, trace, repeated_rows.len(), repeated_cols.len())
    }
}
