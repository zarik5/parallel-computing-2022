use std::{cmp, io, time::Instant};

// https://en.wikipedia.org/wiki/Levenshtein_distance
// Naive Levenshtein distance
fn lev_dist_naive(a: &[u8], b: &[u8]) -> usize {
    if b.is_empty() {
        a.len()
    } else if a.is_empty() {
        b.len()
    } else if a[0] == b[0] {
        lev_dist_naive(&a[1..], &b[1..])
    } else {
        1 + cmp::min(
            usize::min(lev_dist_naive(&a[1..], b), lev_dist_naive(a, &b[1..])),
            lev_dist_naive(&a[1..], &b[1..]),
        )
    }
}

// Optimization 1: copute one row at the time. time O(n*k), space O(2*min(n, k) + 1)
fn lev_dist_opt1<'a>(mut a: &'a [u8], mut b: &'a [u8]) -> usize {
    // Make sure 'a' is always shorter than 'b'
    if a.len() > b.len() {
        (a, b) = (b, a);
    }

    // base case: first row + empty second row
    let mut dist_row1 = (0..a.len() + 1).into_iter().collect::<Vec<_>>();
    let mut dist_row2 = vec![0; a.len() + 1];

    // iteration
    for (b_index, b_item) in b.iter().enumerate() {
        dist_row2[0] = b_index + 1;

        for (a_index, a_item) in a.iter().enumerate() {
            dist_row2[a_index + 1] = if a_item == b_item {
                dist_row1[a_index]
            } else {
                1 + cmp::min(
                    cmp::min(dist_row1[a_index], dist_row1[a_index + 1]),
                    dist_row2[a_index],
                )
            }
        }

        (dist_row1, dist_row2) = (dist_row2, dist_row1);
    }

    dist_row1[a.len()]
}

fn main() -> io::Result<()> {
    let mut a = String::new();
    println!("Insert input 1:");
    io::stdin().read_line(&mut a)?;
    let mut b = String::new();
    println!("Insert input 2:");
    io::stdin().read_line(&mut b)?;

    let start_time = Instant::now();
    let distance = lev_dist_naive(a.as_bytes(), b.as_bytes());
    let computation_time = Instant::now() - start_time;
    println!("Levenshtein distance naive: {distance}\nComputation time: {computation_time:?}");

    let start_time = Instant::now();
    let distance = lev_dist_opt1(a.as_bytes(), b.as_bytes());
    let computation_time = Instant::now() - start_time;
    println!("\nLevenshtein distance opt 1: {distance}\nComputation time: {computation_time:?}");

    Ok(())
}
