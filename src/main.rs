use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use std::io::{self, BufRead, BufReader, Error, Read};

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

#[cfg(test)]
mod test {
    use super::*;
    use stringreader::StringReader;

    fn vec_from_lines(input: &str) -> Vec<String> {
        let mut result = Vec::new();
        for line in input.split("\n") {
            result.push(String::from(line));
        }
        result
    }

    #[test]
    fn test_unique() {
        let input = "a\nb\nc\nd\ne";
        let expected = "a\nb\nc\nd\ne";
        assert_eq!(
            unique(StringReader::new(input)).unwrap(),
            vec_from_lines(expected),
            "keep uniques"
        );

        let input = "a\na\na\na\na";
        let expected = "a";
        assert_eq!(
            unique(StringReader::new(input)).unwrap(),
            vec_from_lines(expected),
            "squash duplicates"
        );

        let input = "a\nb\na\nc\nb\nd\nd\ne";
        let expected = "a\nb\nc\nd\ne";
        assert_eq!(
            unique(StringReader::new(input)).unwrap(),
            vec_from_lines(expected),
            "squash scattered duplicates"
        );

        let input = "b\na\nc\nb\nd\nd\ne";
        let expected = "b\na\nc\nd\ne";
        assert_eq!(
            unique(StringReader::new(input)).unwrap(),
            vec_from_lines(expected),
            "squash scattered duplicate keep order"
        );
    }
}
