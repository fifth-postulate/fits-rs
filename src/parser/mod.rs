//! The parser module is responsible for parsing FITS files.

use std::str;
use std::str::FromStr;
use nom::{is_space, is_digit};
use super::types::{Fits, Header, KeywordRecord, Keyword, Value, BlankRecord};

named!(#[doc = "Will parse data from a FITS file into a `Fits` structure"], pub fits<&[u8], Fits>,
       do_parse!(
           ph: header >>
               many0!(take!(2880)) >>
               (Fits::new(ph))
       ));

named!(header<&[u8], Header>,
       do_parse!(
           records: many0!(keyword_record) >>
               end_record >>
               many0!(blank_record) >>
               (Header::new(records))
       ));

named!(keyword_record<&[u8], KeywordRecord>,
       do_parse!(
           key: keyword  >>
               tag!("= ") >>
           vc: valuecomment >>
               (KeywordRecord::new(key, vc.0, vc.1.map(|c| c.trim() )))
       ));

named!(keyword<&[u8], Keyword>,
       map_res!(
           map_res!(
               take!(8),
               str::from_utf8),
           Keyword::from_str
       ));

named!(valuecomment<&[u8], (Value, Option<&str>)>,
       flat_map!(
           take!(70),
           pair!(
               value,
               opt!(comment)
           )));

named!(value<&[u8], Value>,
       alt!(character_string | logical_constant | real | integer | undefined));

named!(character_string<&[u8], Value>,
       map!(
           map_res!(
               ws!(delimited!(
                   tag!("'"),
                   take_while!(is_allowed_in_character_string),
                   tag!("'")
               )),
               str::from_utf8
           ),
           Value::CharacterString
       ));

fn is_allowed_in_character_string(chr: u8) -> bool {
    is_restricted_ascii(chr) && chr != 39
}

named!(logical_constant<&[u8], Value>,
       map_res!(
           map_res!(
               ws!(alt!(tag!("T") | tag!("F"))),
               str::from_utf8
           ),
           logical_constant_from_str
       ));

/// Problems that could occur when parsing a `str` for a Value::Logical are enumerated here.
pub enum ParseLogicalConstantError {
    /// When encountering anything other than `"T"` or `"F"`.
    UnknownConstant
}

fn logical_constant_from_str(constant: &str) -> Result<Value, ParseLogicalConstantError> {
    match constant {
        "T" => Ok(Value::Logical(true)),
        "F" => Ok(Value::Logical(false)),
        _ => Err(ParseLogicalConstantError::UnknownConstant)
    }
}

named!(integer<&[u8], Value>,
       map!(
           map_res!(
               map_res!(
                   ws!(take_while!(is_digit)), // TODO negative numbers, prefix zeroes
                   str::from_utf8
               ),
               i64::from_str
           ),
           Value::Integer
       ));

named!(real<&[u8], Value>,
       map!(
           map_res!(
               ws!(tuple!(take_while!(is_digit), tag!("."), take_while!(is_digit))),
               tuple_to_f64
           ),
           Value::Real
       ));

/// Reasons for converting to a f64 from a parse triple (left, _, right) to fail.
pub enum RealParseError {
    /// When left is not parse-able as `str`.
    IntegerPartUnparseable,
    /// When right is not parse-able as `str`.
    FractionalPartUnparseable,
    /// When the combination is not a `f64`.
    NotARealNumber,
}

fn tuple_to_f64((left, _, right): (&[u8], &[u8], &[u8])) -> Result<f64, RealParseError> {
    match str::from_utf8(left) {
        Ok(integer_part) => {
            match str::from_utf8(right) {
                Ok(fractional_part) => {
                    let mut number = String::from("");
                    number.push_str(integer_part);
                    number.push_str(".");
                    number.push_str(fractional_part);

                    match f64::from_str(&number) {
                        Ok(result) => Ok(result),
                        Err(_) => Err(RealParseError::NotARealNumber)
                    }
                }
                Err(_) => Err(RealParseError::FractionalPartUnparseable)
            }
        }
        Err(_) => Err(RealParseError::IntegerPartUnparseable)
    }
}

named!(undefined<&[u8], Value>,
       map!(
           take_while!(is_space),
           |_| { Value::Undefined}
       ));

named!(comment<&[u8], &str>,
       map_res!(
           do_parse!(
               tag!("/") >>
                   comment: take_while!(is_restricted_ascii) >>
                   (comment)
           ),
           str::from_utf8
       ));

fn is_restricted_ascii(chr: u8) -> bool {
    32u8 <= chr && chr <= 126u8
}

named!(end_record<&[u8], Keyword>,
       map!(
           flat_map!(
               take!(80),
               pair!(tag!("END"), many0!(tag!(" ")))
           ),
           |_| { Keyword::END }
       ));

named!(blank_record<&[u8], BlankRecord>,
       map!(
           count!(tag!(" "), 80),
           |_| { BlankRecord }
       ));

#[cfg(test)]
mod tests {
    use nom::{IResult};
    use super::super::types::{Fits, Header, KeywordRecord, Keyword, Value, BlankRecord};
    use super::{fits, header, keyword_record, keyword, valuecomment, character_string, logical_constant, real, integer, undefined, end_record, blank_record};

    #[test]
    fn it_should_parse_a_fits_file(){
        let data = include_bytes!("../../assets/images/k2-trappist1-unofficial-tpf-long-cadence.fits");

        let result = fits(data);

        match result {
            IResult::Done(_, f) => {
                assert_eq!(f, Fits::new(long_cadence_header()));
            },
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    #[test]
    fn header_should_parse_a_primary_header(){
        let data = include_bytes!("../../assets/images/k2-trappist1-unofficial-tpf-long-cadence.fits");

        let result = header(&data[0..(2*2880)]);

        match result {
            IResult::Done(_, h) => assert_eq!(h, long_cadence_header()),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    fn long_cadence_header<'a>() -> Header<'a> {
        Header::new(vec!(
            KeywordRecord::new(Keyword::SIMPLE,
                               Value::Logical(true),
                               Option::Some("conforms to FITS standards")),
            KeywordRecord::new(Keyword::BITPIX,
                               Value::Integer(8i64),
                               Option::Some("array data type")),
            KeywordRecord::new(Keyword::NAXIS,
                               Value::Integer(0i64),
                               Option::Some("number of array dimensions")),
            KeywordRecord::new(Keyword::EXTEND,
                               Value::Logical(true),
                               Option::Some("file contains extensions")),
            KeywordRecord::new(Keyword::NEXTEND,
                               Value::Integer(2i64),
                               Option::Some("number of standard extensions")),
            KeywordRecord::new(Keyword::EXTNAME,
                               Value::CharacterString("PRIMARY "),
                               Option::Some("name of extension")),
            KeywordRecord::new(Keyword::EXTVER,
                               Value::Integer(1i64),
                               Option::Some("extension version number (not format version)")),
            KeywordRecord::new(Keyword::ORIGIN,
                               Value::CharacterString("Unofficial data product"),
                               Option::Some("institution responsible for creating this")),
            KeywordRecord::new(Keyword::DATE,
                               Value::CharacterString("2017-03-08"),
                               Option::Some("file creation date.")),
            KeywordRecord::new(Keyword::CREATOR,
                               Value::CharacterString("kadenza "),
                               Option::Some("pipeline job and program u")),
            KeywordRecord::new(Keyword::PROCVER,
                               Value::CharacterString("2.1.dev "),
                               Option::Some("SW version")),
            KeywordRecord::new(Keyword::FILEVER,
                               Value::CharacterString("0.0     "),
                               Option::Some("file format version")),
            KeywordRecord::new(Keyword::TIMVERSN,
                               Value::CharacterString(""),
                               Option::Some("OGIP memo number for file format")),
            KeywordRecord::new(Keyword::TELESCOP,
                               Value::CharacterString("Kepler  "),
                               Option::Some("telescope")),
            KeywordRecord::new(Keyword::INSTRUME,
                               Value::CharacterString("Kepler Photometer"),
                               Option::Some("detector type")),
            KeywordRecord::new(Keyword::OBJECT,
                               Value::CharacterString("EPIC 200164267"),
                               Option::Some("string version of target id")),
            KeywordRecord::new(Keyword::KEPLERID,
                               Value::Integer(200164267i64),
                               Option::Some("unique Kepler target identifier")),
            KeywordRecord::new(Keyword::CHANNEL,
                               Value::Integer(68i64),
                               Option::Some("CCD channel")),
            KeywordRecord::new(Keyword::MODULE,
                               Value::Integer(19i64),
                               Option::Some("CCD module")),
            KeywordRecord::new(Keyword::OUTPUT,
                               Value::Integer(4i64),
                               Option::Some("CCD output")),
            KeywordRecord::new(Keyword::CAMPAIGN,
                               Value::CharacterString(""),
                               Option::Some("Observing campaign number")),
            KeywordRecord::new(Keyword::DATA_REL,
                               Value::CharacterString(""),
                               Option::Some("data release version number")),
            KeywordRecord::new(Keyword::OBSMODE,
                               Value::CharacterString("long cadence"),
                               Option::Some("observing mode")),
            KeywordRecord::new(Keyword::MISSION,
                               Value::CharacterString("K2      "),
                               Option::Some("Mission name")),
            KeywordRecord::new(Keyword::TTABLEID,
                               Value::CharacterString(""),
                               Option::Some("target table id")),
            KeywordRecord::new(Keyword::RADESYS,
                               Value::CharacterString("ICRS    "),
                               Option::Some("reference frame of celestial coordinates")),
            KeywordRecord::new(Keyword::RA_OBJ,
                               Value::CharacterString(""),
                               Option::Some("[deg] right ascension")),
            KeywordRecord::new(Keyword::DEC_OBJ,
                               Value::CharacterString(""),
                               Option::Some("[deg] declination")),
            KeywordRecord::new(Keyword::EQUINOX,
                               Value::Real(2000.0f64),
                               Option::Some("equinox of celestial coordinate system")),
            KeywordRecord::new(Keyword::PMRA,
                               Value::Undefined,
                               Option::Some("[arcsec/yr] RA proper motion")),
            KeywordRecord::new(Keyword::PMDEC,
                               Value::Undefined,
                               Option::Some("[arcsec/yr] Dec proper motion")),
            KeywordRecord::new(Keyword::PMTOTAL,
                               Value::Undefined,
                               Option::Some("[arcsec/yr] total proper motion")),
            KeywordRecord::new(Keyword::PARALLAX,
                               Value::Undefined,
                               Option::Some("[arcsec] parallax")),
            KeywordRecord::new(Keyword::GLON,
                               Value::Undefined,
                               Option::Some("[deg] galactic longitude")),
            KeywordRecord::new(Keyword::GLAT,
                               Value::Undefined,
                               Option::Some("[deg] galactic latitude")),
            KeywordRecord::new(Keyword::GMAG,
                               Value::Undefined,
                               Option::Some("[mag] SDSS g band magnitude")),
            KeywordRecord::new(Keyword::RMAG,
                               Value::Undefined,
                               Option::Some("[mag] SDSS r band magnitude")),
            KeywordRecord::new(Keyword::IMAG,
                               Value::Undefined,
                               Option::Some("[mag] SDSS i band magnitude")),
            KeywordRecord::new(Keyword::ZMAG,
                               Value::Undefined,
                               Option::Some("[mag] SDSS z band magnitude")),
            KeywordRecord::new(Keyword::JMAG,
                               Value::Undefined,
                               Option::Some("[mag] J band magnitude from 2MASS")),
            KeywordRecord::new(Keyword::HMAG,
                               Value::Undefined,
                               Option::Some("[mag] H band magnitude from 2MASS")),
            KeywordRecord::new(Keyword::KMAG,
                               Value::Undefined,
                               Option::Some("[mag] K band magnitude from 2MASS")),
            KeywordRecord::new(Keyword::KEPMAG,
                               Value::Undefined,
                               Option::Some("[mag] Kepler magnitude (Kp)")),
            KeywordRecord::new(Keyword::GRCOLOR,
                               Value::Undefined,
                               Option::Some("[mag] (g-r) color, SDSS bands")),
            KeywordRecord::new(Keyword::JKCOLOR,
                               Value::Undefined,
                               Option::Some("[mag] (J-K) color, 2MASS bands")),
            KeywordRecord::new(Keyword::GKCOLOR,
                               Value::Undefined,
                               Option::Some("[mag] (g-K) color, SDSS g - 2MASS K")),
            KeywordRecord::new(Keyword::TEFF,
                               Value::Undefined,
                               Option::Some("[K] Effective temperature")),
            KeywordRecord::new(Keyword::LOGG,
                               Value::Undefined,
                               Option::Some("[cm/s2] log10 surface gravity")),
            KeywordRecord::new(Keyword::FEH,
                               Value::Undefined,
                               Option::Some("[log10([Fe/H])]  metallicity")),
            KeywordRecord::new(Keyword::EBMINUSV,
                               Value::Undefined,
                               Option::Some("[mag] E(B-V) reddening")),
            KeywordRecord::new(Keyword::AV,
                               Value::Undefined,
                               Option::Some("[mag] A_v extinction")),
            KeywordRecord::new(Keyword::RADIUS,
                               Value::Undefined,
                               Option::Some("[solar radii] stellar radius")),
            KeywordRecord::new(Keyword::TMINDEX,
                               Value::Undefined,
                               Option::Some("unique 2MASS catalog ID")),
            KeywordRecord::new(Keyword::CHECKSUM,
                               Value::CharacterString("7k7A7h637h697h69"),
                               Option::Some("HDU checksum updated 2017-03-08T02:47:56")),
            KeywordRecord::new(Keyword::DATASUM,
                               Value::CharacterString("0       "),
                               Option::Some("data unit checksum updated 2017-03-08T02:47:56")),
        ))
    }

    #[test]
    fn keyword_record_should_parse_a_keyword_record(){
        let data = "OBJECT  = 'EPIC 200164267'     / string version of target id                    "
            .as_bytes();

        let result = keyword_record(data);

        match result {
            IResult::Done(_,k) => {
                assert_eq!(k, KeywordRecord::new(
                    Keyword::OBJECT,
                    Value::CharacterString("EPIC 200164267"),
                    Option::Some("string version of target id")
                ))
            },
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    #[test]
    fn valuecomment_should_parse_a_valuecomment(){
        let data = "'EPIC 200164267'     / string version of target id                    "
            .as_bytes();

        let result = valuecomment(data);

        match result {
            IResult::Done(_, (value, comment)) => {
                assert_eq!(value, Value::CharacterString("EPIC 200164267"));
                assert_eq!(comment, Option::Some(" string version of target id                    "));
            },
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    #[test]
    fn character_string_should_parse_an_quote_delimited_string(){
        let data = "   'EPIC 200164267'   "
            .as_bytes();

        let result = character_string(data);

        match result {
            IResult::Done(_, value) => {
                assert_eq!(value, Value::CharacterString("EPIC 200164267"));
            },
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }


    #[allow(non_snake_case)]
    #[test]
    fn logical_constant_should_parse_an_uppercase_T_or_F(){
        for (constant, boolean) in vec!(("T", true), ("F", false), ("   T ", true)) {
            let data = constant.as_bytes();

            let result = logical_constant(data);

            match result {
                IResult::Done(_, value) => assert_eq!(value, Value::Logical(boolean)),
                IResult::Error(_) => panic!("Did not expect an error"),
                IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
            }
        }
    }

    #[test]
    fn real_should_parse_an_floating_point_number() {
        for (input, f) in vec!(("1.0", 1f64), ("37.0", 37f64), ("51.0", 51f64)) {
            let data = input.as_bytes();

            let result = real(data);

            match result {
                IResult::Done(_, value) => assert_eq!(value, Value::Real(f)),
                IResult::Error(_) => panic!("Did not expect an error"),
                IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
            }
        }
    }

    #[test]
    fn integer_should_parse_an_integer() {
        for (input, n) in vec!(("1", 1i64), ("37", 37i64), ("51", 51i64)) {
            let data = input.as_bytes();

            let result = integer(data);

            match result {
                IResult::Done(_, value) => assert_eq!(value, Value::Integer(n)),
                IResult::Error(_) => panic!("Did not expect an error"),
                IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
            }
        }
    }

    #[test]
    fn undefined_should_parse_any_amount_of_whitespace() {
        for input in vec!(" ", "\t", "    \t   ") {
            let data = input.as_bytes();

            let result = undefined(data);

            match result {
                IResult::Done(_, value) => assert_eq!(value, Value::Undefined),
                IResult::Error(_) => panic!("Did not expect an error"),
                IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
            }
        }
    }

    #[test]
    fn keyword_should_parse_a_keyword(){
        let data = "OBJECT  "
            .as_bytes();

        let result = keyword(data);

        match result {
            IResult::Done(_, keyword) => assert_eq!(keyword, Keyword::OBJECT),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn end_record_should_parse_an_END_record(){
        let data = "END                                                                             "
            .as_bytes();

        let result = end_record(data);

        match result {
            IResult::Done(_, keyword) => assert_eq!(keyword, Keyword::END),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    #[test]
    #[allow(non_snake_case)]
    fn blank_record_should_parse_a_BLANK_record(){
        let data = "                                                                                "
            .as_bytes();

        let result = blank_record(data);

        match result {
            IResult::Done(_, record) => assert_eq!(record, BlankRecord),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }
}
