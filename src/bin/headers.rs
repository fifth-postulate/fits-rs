extern crate nom;
extern crate fits_rs;

use std::env;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use nom::IResult;
use fits_rs::parser::fits;
use fits_rs::types::Header;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let header_index = u64::from_str(&args[2]).expect("second argument should be a non-negative number");

    let mut f = File::open(filename).expect("file not found");
    let mut buffer: Vec<u8> = vec!();
    let _ = f.read_to_end(&mut buffer);

    let result = fits(&buffer);

    match result {
        IResult::Done(_, trappist1) => {  //Done produces error[E0599]=variant or associated item not found in `Result<(_, _), nom::Err<_>>`
            let header: &Header = if header_index == 0 {
                &trappist1.primary_hdu.header
            } else {
                &trappist1.extensions[0].header
            };

            for ref record in &header.keyword_records {
                println!("{}", record);
            }
        },
        _ => panic!("Whoops, something went wrong")
    }
}
