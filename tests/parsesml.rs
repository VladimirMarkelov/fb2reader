// use fb2reader::FB2Reader;
use std::path::PathBuf;
use quick_xml::{events::Event, reader::Reader};

#[test]
fn parse_xml() {
	// let path = PathBuf::from("test.fb2");
	// let mut rd = Reader::from_file(path).unwrap();
	// let mut buf = Vec::new();
	// loop {
	// 	match rd.read_event_into(&mut buf) {
	// 		Err(er) => {
	// 			println!("Err: {:?}", er);
	// 			break;
	// 		},
	// 		Ok(Event::Eof) => break,
	// 		Ok(ev) => {
	// 			println!("ITEM {:?}", ev);
	// 		}
	// 	}
	// 	buf.clear();
	// }
}
