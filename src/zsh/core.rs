const META: u8 = 0x83;
const META_MASK: u8 = 0b100000;
const NULL: u8 = 0x0;
const MARKER: u8 = 0xA2;
const POUND: u8 = 0x84;
const LAST_NORMAL_TOK: u8 = 0x9c;
const SNULL: u8 = 0x9d;
const NULARG: u8 = 0xA1;

const fn meta_chars_len() -> usize {
    let mut len: usize = 3;
    len += LAST_NORMAL_TOK as usize - POUND as usize + 1;
    len += NULARG as usize - SNULL as usize + 1;
    len
}

const META_CHARS: [u8; meta_chars_len()] = {
    // original: inittyptab
    let mut chars = [0u8; meta_chars_len()];

    chars[0] = NULL;
    chars[1] = META;
    chars[2] = MARKER;

    let mut i = 3usize;

    let mut min = POUND;
    let mut max = LAST_NORMAL_TOK;
    let mut current = min;

    loop {
        if current > max {
            break;
        }
        chars[i] = current;
        current += 1;
        i += 1;
    }

    min = SNULL;
    max = NULARG;
    current = min;

    loop {
        if current > max {
            break;
        }
        chars[i] = current;
        current += 1;
        i += 1;
    }

    chars
};

// original: imeta
fn is_meta_char(char: u8) -> bool {
    META_CHARS.contains(&char)
}

pub fn metafy(str: &[u8]) -> Vec<u8> {
    let mut buf = vec![];

    for &c in str {
        if !is_meta_char(c) {
            buf.push(c);
            continue;
        }

        buf.push(META);
        buf.push(c ^ META_MASK);
    }

    buf
}

pub fn unmetafy(str: &[u8]) -> Vec<u8> {
    let mut buf = vec![];

    let mut skip = false;
    for (i, &c) in str.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }

        if c != META {
            buf.push(c);
            continue;
        }

        let next = *str
            .get(i + 1)
            .expect("META should not be placed at the end of the history entry");
        buf.push(next ^ META_MASK);
        skip = true;
    }

    buf
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use rstest_reuse::{self, *};

    use super::*;

    fn from_string(str: &str) -> Vec<u8> {
        str.to_string().into_bytes()
    }

    #[template]
    #[rstest(
      _name, metafied, unmetafied,
      case("simple", from_string("echo 123"), from_string("echo 123")),
      case("dragon", vec![240, 131, 191, 131, 176, 178], from_string("🐲")),
      case(
        "family",
        vec![
          240, 131, 191, 131,
          177, 168, 226, 128,
          131, 173, 240, 131,
          191, 131, 177, 168,
          226, 128, 131, 173,
          240, 131, 191, 131,
          177, 167, 226, 128,
          131, 173, 240, 131,
          191, 131, 177, 166,
        ],
        from_string("👨‍👨‍👧‍👦"),
      ),
      ::trace
    )]
    fn meta_cases(_name: &str, metafied: Vec<u8>, unmetafied: Vec<u8>) {}

    #[apply(meta_cases)]
    fn test_metafy(_name: &str, metafied: Vec<u8>, unmetafied: Vec<u8>) {
        assert_eq!(metafy(&unmetafied), metafied);
    }

    #[apply(meta_cases)]
    fn test_unmetafy(_name: &str, metafied: Vec<u8>, unmetafied: Vec<u8>) {
        assert_eq!(unmetafy(&metafied), unmetafied);
    }
}
