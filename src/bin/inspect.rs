use std::env;
use std::fs::File;
use std::io::Read;
use std::str;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");
    let mut buffer: Vec<u8> = vec!();
    let _ = f.read_to_end(&mut buffer);

    let result: &[u8]= &buffer;

    println!("{:?}", str::from_utf8(&result[2*2880..10*2880]).expect("should be utf8"));
}

