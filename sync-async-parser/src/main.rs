use std::io::Cursor;

use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;


enum ParserState<T> {
    NeedBytes(usize),
    Done(T),
}

trait Parser<T> {
    fn read(&mut self, input: &[u8]) -> ParserState<T>;
}

struct IntParser {
    bytes: Vec<u8>,
}

impl IntParser {
    fn new() -> IntParser {
        IntParser {
            bytes: vec![]
        }
    }
}

impl Parser<i32> for IntParser {
    fn read(&mut self, input: &[u8]) -> ParserState<i32> {
        self.bytes.extend_from_slice(input);
        if self.bytes.len() == 4 {
            let mut cursor = Cursor::new(self.bytes.clone());
            let int = ReadBytesExt::read_i32::<BigEndian>(&mut cursor).unwrap();
            ParserState::Done(int)
        } else {
            ParserState::NeedBytes(4 - self.bytes.len())
        }
    }
}

fn parse_sync<T>(stream: &mut impl std::io::Read, parser: &mut impl Parser<T>) -> Result<T> {
    let mut state = parser.read(&[]);
    loop {
        match state {
            ParserState::Done(res) => return Ok(res),
            ParserState::NeedBytes(num) => {
                let mut buf = vec![0; num];
                stream.read_exact(buf.as_mut()).unwrap();
                state = parser.read(&buf[..]);
            }
        }
    }
}

async fn parse_async<T>(stream: &mut (impl AsyncRead + Unpin), parser: &mut impl Parser<T>) -> Result<T> {
    let mut state = parser.read(&[]);
    loop {
        match state {
            ParserState::Done(res) => return Ok(res),
            ParserState::NeedBytes(num) => {
                let mut buf = vec![0; num];
                stream.read_exact(buf.as_mut()).await.unwrap();
                state = parser.read(&buf[..]);
            }
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let num: i32 = 55;
    let bytes = num.to_be_bytes();

    let mut cursor = Cursor::new(bytes);
    let parsed = parse_sync(&mut cursor, &mut IntParser::new()).unwrap();
    println!("sync parsed {}", parsed);

    let mut cursor = Cursor::new(bytes);
    let parsed = parse_async(&mut cursor, &mut IntParser::new()).await.unwrap();
    println!("async parsed {}", parsed);

    Ok(())
}
