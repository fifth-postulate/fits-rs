# fits-rs [![Build Status](https://travis-ci.org/fifth-postulate/fits-rs.svg?branch=master)](https://travis-ci.org/fifth-postulate/fits-rs) [![Crate](https://img.shields.io/crates/v/fits-rs.svg)](https://crates.io/crates/fits-rs)
FITS encoder and decoder in Rust.

Make sure to check out the [documentation][] of this crate.

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
The headers of FITS files are in ASCII. This means they can be read. The
following command will output the primary header for the FITS file in the
repository.

```
head --bytes=5760 assets/images/k2-trappist1-unofficial-tpf-long-cadence.fits | sed -e "s/.\{80\}/&\n/g"
```

[fits]: https://en.wikipedia.org/wiki/FITS
[reference]: https://fits.gsfc.nasa.gov/fits_standard.html
[fits-homepage]: https://fits.gsfc.nasa.gov/fits_standard.html
[documentation]: http://fifth-postulate/fits-rs/ 
