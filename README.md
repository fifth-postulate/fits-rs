# fits-rs [![Build Status](https://travis-ci.org/fifth-postulate/fits-rs.svg?branch=master)](https://travis-ci.org/fifth-postulate/fits-rs) [![Coverage Status](https://coveralls.io/repos/github/fifth-postulate/fits-rs/badge.svg?branch=master)](https://coveralls.io/github/fifth-postulate/fits-rs?branch=master) [![Crate](https://img.shields.io/crates/v/fits-rs.svg)](https://crates.io/crates/fits-rs)
FITS encoder and decoder in Rust.

Make sure to check out the [documentation][] of this crate. The [build][] can be
followed as well. The [coverage report][coverage] is provided by [coveralls][coveralls].

## FITS
The *Flexible Image Transport System* ([FITS][fits]) is 

> an open standard defining a digital file format useful for storage,
> transmission and processing of scientific and other images. 

The [reference documentation][reference] on the FITS standard can be found an
NASA's [FITS pages][fits-homepage]. You can get a copy by executing the
following command:

```plain
wget --output-document=fits-reference.pdf "https://www.aanda.org/articles/aa/pdf/2010/16/aa15362-10.pdf"
```

### Reading Primary Header
Even though the headers of FITS files are in ASCII, you can use this crate to
read the primary header.

```rust
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let header_index = u64::from_str(&args[2]).expect("should be a non-negative number");

    let mut f = File::open(filename).expect("file not found");
    let mut buffer: Vec<u8> = vec!();
    let _ = f.read_to_end(&mut buffer);

    let result = fits(&buffer);

    match result {
        IResult::Done(_, trappist1) => {
            let header: &Header = if header_index = 0 {
                &trappist1.primary_hdu.header
            } else {
                &trappist1.extensions[header_index].header
            }

            for record in header.keyword_records {
                println!("{:?}", record);
            }
        },
        _ => panic!("Whoops, something went wrong")
    }
```

You can find this binary in [`src/bin/headers.rs`](https://github.com/fifth-postulate/fits-rs/blob/master/src/bin/headers.rs).

Unfortunately, some extensions are in binary.

[fits]: https://en.wikipedia.org/wiki/FITS
[reference]: https://fits.gsfc.nasa.gov/fits_standard.html
[fits-homepage]: https://fits.gsfc.nasa.gov/fits_standard.html
[documentation]: http://fifth-postulate/fits-rs/ 
[build]: https://travis-ci.org/fifth-postulate/fits-rs
[coverage]: https://coveralls.io/github/fifth-postulate/fits-rs?branch=master
[coveralls]: https://coveralls.io/
