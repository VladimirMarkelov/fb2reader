use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use quick_xml::{events::Event, reader::Reader};

use crate::err::FB2Error;
use crate::types;

pub struct FB2Reader {
    buf: Vec<u8>,
    reader: Reader<BufReader<File>>,
}

impl FB2Reader {
    pub fn from_file(path: &Path) -> Result<Self, FB2Error> {
        match Reader::from_file(path) {
            Ok(r) => Ok(FB2Reader{reader: r, buf: Vec::new()}),
            Err(_) => Err(FB2Error::OpenFailed),
        }
    }

    pub fn parse_book(&mut self) -> Result<types::Book, FB2Error> {
        let mut book: types::Book = Default::default();
        loop {
            let ev = match self.reader.read_event_into(&mut self.buf) {
                Err(er) => return Err(FB2Error::ParseFailed(er.to_string())),
                Ok(e) => e,
            };
            match ev {
                Event::Eof => return Ok(book),
                _ => {},
            }
        }
    }
}
