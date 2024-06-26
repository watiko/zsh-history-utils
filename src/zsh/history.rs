use std::io::{BufRead, BufReader, Read};

use eyre::{eyre, Result};
use nom::bytes::complete::{tag, take, take_until, take_while};
use nom::character::is_digit;
use nom::combinator::map_res;
use nom::multi::many0;
use nom::sequence::{preceded, terminated, tuple};
use serde::{Deserialize, Serialize};

use super::core::{metafy, unmetafy};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub start_time: u64,
    pub finish_time: u64,
    pub command: String,
}

fn has_slash_with_space(input: &[u8]) -> bool {
    let mut has_space = false;
    let mut iter = input.iter().rev();
    // skip last \n
    if iter.next() != Some(&b'\n') {
        return false;
    }
    for &c in iter {
        match c {
            b' ' => has_space = true,
            b'\\' => return has_space,
            _ => return false,
        };
    }

    false
}

fn parse_command_lines(input: &[u8]) -> nom::IResult<&[u8], Vec<u8>> {
    fn command_line(input: &[u8]) -> nom::IResult<&[u8], &[u8]> {
        terminated(take_until(&b"\\\n"[..]), take(2usize))(input)
    }

    fn last_command_line(input: &[u8]) -> nom::IResult<&[u8], &[u8]> {
        if has_slash_with_space(input) {
            return terminated(take(input.len() - 2), tag(&b" \n"[..]))(input);
        }

        terminated(take_until(&b"\n"[..]), tag(b"\n"))(input)
    }

    let (input, (lines, line)) = tuple((many0(command_line), last_command_line))(input)?;

    let mut buf = vec![];
    for line in lines {
        buf.extend_from_slice(line);
        buf.push(b'\n');
    }
    buf.extend_from_slice(line);

    Ok((input, buf))
}

fn parse_history_entry(input: &[u8]) -> nom::IResult<&[u8], (u64, u64, Vec<u8>)> {
    fn as_u64(buf: &[u8]) -> Result<u64> {
        let str = std::str::from_utf8(buf)?;
        let value = str.parse::<u64>()?;
        Ok(value)
    }

    let (input, (start_time, duration, command)) = tuple((
        preceded(tag(&b": "[..]), map_res(take_while(is_digit), as_u64)),
        preceded(tag(b":"), map_res(take_while(is_digit), as_u64)),
        preceded(tag(b";"), parse_command_lines),
    ))(input)?;

    Ok((input, (start_time, duration, command)))
}

impl HistoryEntry {
    pub fn parse(line: &[u8]) -> Result<Self> {
        let (line, (start_time, duration, command)) = parse_history_entry(line)
            .map_err(|err| eyre!("failed to parse as HistoryEntry: {}", err))?;

        if !line.is_empty() {
            return Err(eyre!("parse succeeded, but input remains: {:?}", line));
        }

        let entry = HistoryEntry {
            command: String::from_utf8(unmetafy(&command))?,
            start_time,
            finish_time: start_time + duration,
        };

        Ok(entry)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let duration = self.finish_time - self.start_time;

        let mut read_buf = format!(": {}:{};", self.start_time, duration).into_bytes();
        read_buf.extend_from_slice(&metafy(self.command.as_bytes()));

        let mut buf = vec![];
        let mut end_backslashed = false;
        for c in read_buf {
            end_backslashed = c == b'\\' || (end_backslashed && c == b' ');

            if c == b'\n' {
                buf.push(b'\\');
            }
            buf.push(c);
        }

        if end_backslashed {
            buf.push(b' ');
        }
        buf.push(b'\n');

        buf
    }
}

#[derive(Debug)]
pub struct HistoryLines<R> {
    buf: BufReader<R>,
}

impl<R: Read> HistoryLines<R> {
    pub fn new(inner: R) -> HistoryLines<R> {
        Self {
            buf: BufReader::new(inner),
        }
    }
}

impl<R: Read> Iterator for HistoryLines<R> {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = vec![];

        loop {
            let read_bytes = match self.buf.read_until(b'\n', &mut buf) {
                Ok(v) => v,
                Err(err) => return Some(Err(eyre!("iteration interrupted: {:?}", err))),
            };
            if read_bytes == 0 {
                if buf.is_empty() {
                    return None;
                }
                break;
            }

            let more = buf.ends_with(&[b'\\', b'\n']);
            if !more {
                break;
            }
        }

        Some(Ok(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    #[derive(Debug)]
    enum Line {
        Plain(String),
        Bytes(Vec<u8>),
    }

    impl Line {
        fn into_bytes(self) -> Vec<u8> {
            match self {
                Line::Plain(line) => line.into_bytes(),
                Line::Bytes(line) => line,
            }
        }
    }

    #[template]
    #[rstest(
      _name, entry, line,
      case(
        "simple",
        HistoryEntry {
          command: "echo 123456".to_string(),
          start_time: 123,
          finish_time: 456,
        },
        Line::Plain(": 123:333;echo 123456\n".to_string()),
      ),
      case(
        "simple2",
        HistoryEntry {
          command: "sleep 2".to_string(),
          start_time: 1639320933,
          finish_time: 1639320935,
        },
        Line::Plain(": 1639320933:2;sleep 2\n".to_string()),
      ),
      case(
        "simple3",
        HistoryEntry {
          command: "echo \\".to_string(),
          start_time: 1639322528,
          finish_time: 1639322528,
        },
        Line::Plain(": 1639322528:0;echo \\ \n".to_string()),
      ),
      case(
        "multi line entry",
        HistoryEntry {
          command: "echo one \\\n  echo two".to_string(),
          start_time: 1111,
          finish_time: 1111,
        },
        Line::Plain(": 1111:0;echo one \\\\\n  echo two\n".to_string()),
      ),
      case(
        "multi line entry2",
        HistoryEntry {
          command: "echo \n1\n2\\  ".to_string(),
          start_time: 1,
          finish_time: 1,
        },
        Line::Plain(": 1:0;echo \\\n1\\\n2\\   \n".to_string()),
      ),
      case(
        "meta",
        HistoryEntry {
          command: "echo ペンギン".to_string(),
          start_time: 1,
          finish_time: 1,
        },
        Line::Bytes(vec![
          58,  32,  49,  58,  48,  59, 101,
          99, 104, 111,  32, 227, 131, 163,
          131, 186, 227, 131, 163, 179, 227,
          130, 174, 227, 131, 163, 179,  10
        ]),
      ),
      ::trace
    )]
    fn history_entry_cases(_name: &str, entry: HistoryEntry, line: Line) {}

    #[apply(history_entry_cases)]
    fn test_entry_to_line(_name: &str, entry: HistoryEntry, line: Line) {
        let entry_bytes = entry.to_bytes();
        assert_eq!(entry_bytes, line.into_bytes());
    }

    #[apply(history_entry_cases)]
    fn test_line_to_entry(_name: &str, entry: HistoryEntry, line: Line) {
        let parsed_entry = HistoryEntry::parse(&line.into_bytes()).unwrap();
        assert_eq!(parsed_entry, entry);
    }

    #[rstest(
      _name, histories, lines,
      case(
        "simple",
        [
          ": 1639324265:0;echo 1 2 3",
          ": 1639324275:0;echo \"\"",
          ": 1639324281:0;echo {1,2,3}",
          "",
        ].join("\n"),
        vec![
          Line::Plain(": 1639324265:0;echo 1 2 3\n".to_string()),
          Line::Plain(": 1639324275:0;echo \"\"\n".to_string()),
          Line::Plain(": 1639324281:0;echo {1,2,3}\n".to_string()),
        ],
      ),
      case(
        "simple",
        [
          ": 1639320933:0;echo one \\ ",
          ": 1639322528:0;echo two \\\\ ",
          ": 1639320933:0;echo one \\",
          ": 1639322528:0;echo two \\\\ ",
          ": 1639322832:0;echo 2 \\\\",
          " 2 \\\\",
          " 1 \\ ",
          ": 1639322528:0;echo",
          "",
        ].join("\n"),
        vec![
          Line::Plain(": 1639320933:0;echo one \\ \n".to_string()),
          Line::Plain(": 1639322528:0;echo two \\\\ \n".to_string()),
          Line::Plain(": 1639320933:0;echo one \\\n: 1639322528:0;echo two \\\\ \n".to_string()),
          Line::Plain(": 1639322832:0;echo 2 \\\\\n 2 \\\\\n 1 \\ \n".to_string()),
          Line::Plain(": 1639322528:0;echo\n".to_string()),
        ],
      ),
      ::trace
    )]
    fn test_history_lines(_name: &str, histories: String, lines: Vec<Line>) {
        let iterated_lines = HistoryLines::new(histories.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .map(|line| std::str::from_utf8(&line).unwrap().to_string())
            .collect::<Vec<_>>();
        let lines = lines
            .into_iter()
            .map(|line| line.into_bytes())
            .map(|line| std::str::from_utf8(&line).unwrap().to_string())
            .collect::<Vec<_>>();

        assert_eq!(iterated_lines, lines);
    }
}
