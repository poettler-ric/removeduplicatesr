use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
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
        let line = line?;

        let mut hasher = DefaultHasher::new();
        hasher.write(line.as_bytes());
        let hash = hasher.finish();

        if !seen.contains(&hash) {
            seen.insert(hash);
            lines.push(line);
        }
    }
    Ok(lines)
}

fn main() {
    let lines = match unique(io::stdin()) {
        Ok(v) => v,
        Err(e) => panic!("error while reading input: {:?}", e),
    };
    for line in lines {
        println!("{}", line);
    }
}
