use std::io;

// https://en.wikipedia.org/wiki/Levenshtein_distance
fn levenshtein_distance(a: &[u8], b: &[u8]) -> usize {
    if b.is_empty() {
        a.len()
    } else if a.is_empty() {
        b.len()
    } else if a[0] == b[0] {
        levenshtein_distance(&a[1..], &b[1..])
    } else {
        1 + usize::min(
            usize::min(
                levenshtein_distance(&a[1..], b),
                levenshtein_distance(a, &b[1..]),
            ),
            levenshtein_distance(&a[1..], &b[1..]),
        )
    }
}

fn main() -> io::Result<()> {
    let mut a = String::new();
    println!("Insert input 1:");
    io::stdin().read_line(&mut a)?;

    let mut b = String::new();
    println!("Insert input 2:");
    io::stdin().read_line(&mut b)?;

    let distance = levenshtein_distance(a.as_bytes(), b.as_bytes());
    println!("Levenshtein distance: {distance}");

    Ok(())
}
