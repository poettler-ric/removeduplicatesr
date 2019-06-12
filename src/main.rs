use std::collections::HashSet;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;

// TODO: filter iterator
// TODO: use .map() on iterator?
// TODO: use .dedup() on a vector? itertools?

fn unique(input: impl Read) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(input);
    let mut seen = HashSet::new();
    let mut lines = Vec::new();

    for line in reader.lines() {
        let line = match line {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        if !seen.contains(&line) {
            // FIXME: remove clone
            seen.insert(line.clone());
            lines.push(line);
        }
    }
    Ok(lines)
}

fn main() -> Result<(), Error> {
    for line in unique(io::stdin())? {
        println!("{}", line);
    }
    Ok(())
}
