use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let low = usize::from_str(&args[2]).expect("second argument should be a number");
    let high = usize::from_str(&args[3]).expect("thirs argument should be a number");

    let mut f = File::open(filename).expect("file not found");
    let mut buffer: Vec<u8> = vec!();
    let _ = f.read_to_end(&mut buffer);

    let result: &[u8]= &buffer;

    println!("{:?}", str::from_utf8(&result[low..high]).expect("should be utf8"));
}

