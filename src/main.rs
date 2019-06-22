use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::Hasher;
use std::io::{self, BufRead, BufReader, Error, Read};

fn hash_string(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    hasher.write(s.as_bytes());
    hasher.finish()
}

fn unique(input: impl Read) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(input);
    let mut seen = HashSet::new();

    let lines: Result<Vec<_>, _> = reader
        .lines()
        .filter(|x| {
            let x = match x {
                Ok(v) => v,
                Err(_) => return true, // don't handle error - let collect return it
            };

            let hash = hash_string(&x);
            if seen.contains(&hash) {
                return false;
            }
            seen.insert(hash);
            true
        })
        .collect();
    lines
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
