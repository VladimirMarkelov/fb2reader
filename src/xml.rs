use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use quick_xml::{events::Event, reader::Reader};

use crate::err::FB2Error;
use crate::types;

pub struct FB2Reader {
    done: bool,
    buf: Vec<u8>,
    reader: Reader<BufReader<File>>,
}

impl FB2Reader {
    pub fn from_file(path: &Path) -> Result<Self, FB2Error> {
        match Reader::from_file(path) {
            Ok(r) => Ok(FB2Reader{done: false, reader: r, buf: Vec::new()}),
            Err(_) => Err(FB2Error::OpenFailed),
        }
    }
}

impl Iterator for FB2Reader {
    type Item = types::Element;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let ev = match self.reader.read_event_into(&mut self.buf) {
            Err(_) => { self.done = true; return None; },
            Ok(e) => e,
        };
        match ev {
            Event::Eof => { self.done = true; None },
            _ => None,
        }
    }
}
