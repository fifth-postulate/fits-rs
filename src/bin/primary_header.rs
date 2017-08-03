extern crate nom;
extern crate fits_rs;

use std::env;
use std::fs::File;
use std::io::Read;
use nom::IResult;
use fits_rs::parser::fits;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut buffer: Vec<u8> = vec!();
    let _ = f.read_to_end(&mut buffer);

    let result = fits(&buffer);

    match result {
        IResult::Done(_, trappist1) => {
            for record in trappist1.primary_header.keyword_records {
                println!("{}", record);
            }
        },
        _ => panic!("Whoops, something went wrong")
    }
}
