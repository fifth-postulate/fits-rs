//! The parser module is responsible for parsing FITS files.

use std::str;
use std::str::FromStr;
use nom::{is_space, is_digit};
use super::types::{Fits, PrimaryHeader, KeywordRecord, Keyword, Value, BlankRecord};

named!(#[doc = "Will parse data from a FITS file into a `Fits` structure"], pub fits<&[u8], Fits>,
       do_parse!(
           ph: primary_header >>
               many0!(take!(2880)) >>
               (Fits::new(ph))
       ));

named!(primary_header<&[u8], PrimaryHeader>,
       do_parse!(
           records: many0!(keyword_record) >>
               end_record >>
               many0!(blank_record) >>
               (PrimaryHeader::new(records))
       ));

named!(keyword_record<&[u8], KeywordRecord>,
       do_parse!(
           key: keyword  >>
               tag!("= ") >>
           vc: valuecomment >>
               (KeywordRecord::new(key, vc.0, vc.1))
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
       alt!(character_string | logical_constant | integer | raw));

named!(raw<&[u8], Value>,
       map!(
           map_res!(
               is_not!("/"), // TODO Differentiate on the possible value types
               str::from_utf8
           ),
           Value::Raw
       ));

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
                   ws!(take_while!(is_digit)), // TODO negative numbers, trailing zeroes
                   str::from_utf8
               ),
               i64::from_str
           ),
           Value::Integer
       ));

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
    use super::super::types::{Fits, PrimaryHeader, KeywordRecord, Keyword, Value, BlankRecord};
    use super::{fits, primary_header, keyword_record, keyword, valuecomment, character_string, logical_constant, integer, undefined, end_record, blank_record};

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
    fn primary_header_should_parse_a_primary_header(){
        let data = include_bytes!("../../assets/images/k2-trappist1-unofficial-tpf-long-cadence.fits");

        let result = primary_header(&data[0..(2*2880)]);

        match result {
            IResult::Done(_, h) => assert_eq!(h, long_cadence_header()),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    fn long_cadence_header<'a>() -> PrimaryHeader<'a> {
        PrimaryHeader::new(vec!(
            KeywordRecord::new(Keyword::SIMPLE,
                               Value::Logical(true),
                               Option::Some(" conforms to FITS standards                     ")),
            KeywordRecord::new(Keyword::BITPIX,
                               Value::Integer(8i64),
                               Option::Some(" array data type                                ")),
            KeywordRecord::new(Keyword::NAXIS,
                               Value::Integer(0i64),
                               Option::Some(" number of array dimensions                     ")),
            KeywordRecord::new(Keyword::EXTEND,
                               Value::Logical(true),
                               Option::Some(" file contains extensions                       ")),
            KeywordRecord::new(Keyword::NEXTEND,
                               Value::Integer(2i64),
                               Option::Some(" number of standard extensions                  ")),
            KeywordRecord::new(Keyword::EXTNAME,
                               Value::CharacterString("PRIMARY "),
                               Option::Some(" name of extension                              ")),
            KeywordRecord::new(Keyword::EXTVER,
                               Value::Integer(1i64),
                               Option::Some(" extension version number (not format version)  ")),
            KeywordRecord::new(Keyword::ORIGIN,
                               Value::CharacterString("Unofficial data product"),
                               Option::Some(" institution responsible for creating this ")),
            KeywordRecord::new(Keyword::DATE,
                               Value::CharacterString("2017-03-08"),
                               Option::Some(" file creation date.                            ")),
            KeywordRecord::new(Keyword::CREATOR,
                               Value::CharacterString("kadenza "),
                               Option::Some(" pipeline job and program u                     ")),
            KeywordRecord::new(Keyword::PROCVER,
                               Value::CharacterString("2.1.dev "),
                               Option::Some(" SW version                                     ")),
            KeywordRecord::new(Keyword::FILEVER,
                               Value::CharacterString("0.0     "),
                               Option::Some(" file format version                            ")),
            KeywordRecord::new(Keyword::TIMVERSN,
                               Value::CharacterString(""),
                               Option::Some(" OGIP memo number for file format                                 ")),
            KeywordRecord::new(Keyword::TELESCOP,
                               Value::CharacterString("Kepler  "),
                               Option::Some(" telescope                                      ")),
            KeywordRecord::new(Keyword::INSTRUME,
                               Value::CharacterString("Kepler Photometer"),
                               Option::Some(" detector type                                  ")),
            KeywordRecord::new(Keyword::OBJECT,
                               Value::CharacterString("EPIC 200164267"),
                               Option::Some(" string version of target id                    ")),
            KeywordRecord::new(Keyword::KEPLERID,
                               Value::Integer(200164267i64),
                               Option::Some(" unique Kepler target identifier                ")),
            KeywordRecord::new(Keyword::CHANNEL,
                               Value::Integer(68i64),
                               Option::Some(" CCD channel                                    ")),
            KeywordRecord::new(Keyword::MODULE,
                               Value::Integer(19i64),
                               Option::Some(" CCD module                                     ")),
            KeywordRecord::new(Keyword::OUTPUT,
                               Value::Integer(4i64),
                               Option::Some(" CCD output                                     ")),
            KeywordRecord::new(Keyword::CAMPAIGN,
                               Value::CharacterString(""),
                               Option::Some(" Observing campaign number                                        ")),
            KeywordRecord::new(Keyword::DATA_REL,
                               Value::CharacterString(""),
                               Option::Some(" data release version number                                      ")),
            KeywordRecord::new(Keyword::OBSMODE,
                               Value::CharacterString("long cadence"),
                               Option::Some(" observing mode                                 ")),
            KeywordRecord::new(Keyword::MISSION,
                               Value::CharacterString("K2      "),
                               Option::Some(" Mission name                                   ")),
            KeywordRecord::new(Keyword::TTABLEID,
                               Value::CharacterString(""),
                               Option::Some(" target table id                                                  ")),
            KeywordRecord::new(Keyword::RADESYS,
                               Value::CharacterString("ICRS    "),
                               Option::Some(" reference frame of celestial coordinates       ")),
            KeywordRecord::new(Keyword::RA_OBJ,
                               Value::CharacterString(""),
                               Option::Some(" [deg] right ascension                                            ")),
            KeywordRecord::new(Keyword::DEC_OBJ,
                               Value::CharacterString(""),
                               Option::Some(" [deg] declination                                                ")),
            KeywordRecord::new(Keyword::EQUINOX,
                               Value::Integer(2000i64), // TODO should be Real(2000.0f64)
                               Option::None), //Some(" equinox of celestial coordinate system         ")),
            KeywordRecord::new(Keyword::PMRA,
                               Value::Raw(" "),
                               Option::Some(" [arcsec/yr] RA proper motion                                       ")),
            KeywordRecord::new(Keyword::PMDEC,
                               Value::Raw(" "),
                               Option::Some(" [arcsec/yr] Dec proper motion                                      ")),
            KeywordRecord::new(Keyword::PMTOTAL,
                               Value::Raw(" "),
                               Option::Some(" [arcsec/yr] total proper motion                                    ")),
            KeywordRecord::new(Keyword::PARALLAX,
                               Value::Raw(" "),
                               Option::Some(" [arcsec] parallax                                                  ")),
            KeywordRecord::new(Keyword::GLON,
                               Value::Raw(" "),
                               Option::Some(" [deg] galactic longitude                                           ")),
            KeywordRecord::new(Keyword::GLAT,
                               Value::Raw(" "),
                               Option::Some(" [deg] galactic latitude                                            ")),
            KeywordRecord::new(Keyword::GMAG,
                               Value::Raw(" "),
                               Option::Some(" [mag] SDSS g band magnitude                                        ")),
            KeywordRecord::new(Keyword::RMAG,
                               Value::Raw(" "),
                               Option::Some(" [mag] SDSS r band magnitude                                        ")),
            KeywordRecord::new(Keyword::IMAG,
                               Value::Raw(" "),
                               Option::Some(" [mag] SDSS i band magnitude                                        ")),
            KeywordRecord::new(Keyword::ZMAG,
                               Value::Raw(" "),
                               Option::Some(" [mag] SDSS z band magnitude                                        ")),
            KeywordRecord::new(Keyword::JMAG,
                               Value::Raw(" "),
                               Option::Some(" [mag] J band magnitude from 2MASS                                  ")),
            KeywordRecord::new(Keyword::HMAG,
                               Value::Raw(" "),
                               Option::Some(" [mag] H band magnitude from 2MASS                                  ")),
            KeywordRecord::new(Keyword::KMAG,
                               Value::Raw(" "),
                               Option::Some(" [mag] K band magnitude from 2MASS                                  ")),
            KeywordRecord::new(Keyword::KEPMAG,
                               Value::Raw(" "),
                               Option::Some(" [mag] Kepler magnitude (Kp)                                        ")),
            KeywordRecord::new(Keyword::GRCOLOR,
                               Value::Raw(" "),
                               Option::Some(" [mag] (g-r) color, SDSS bands                                      ")),
            KeywordRecord::new(Keyword::JKCOLOR,
                               Value::Raw(" "),
                               Option::Some(" [mag] (J-K) color, 2MASS bands                                     ")),
            KeywordRecord::new(Keyword::GKCOLOR,
                               Value::Raw(" "),
                               Option::Some(" [mag] (g-K) color, SDSS g - 2MASS K                                ")),
            KeywordRecord::new(Keyword::TEFF,
                               Value::Raw(" "),
                               Option::Some(" [K] Effective temperature                                          ")),
            KeywordRecord::new(Keyword::LOGG,
                               Value::Raw(" "),
                               Option::Some(" [cm/s2] log10 surface gravity                                      ")),
            KeywordRecord::new(Keyword::FEH,
                               Value::Raw(" "),
                               Option::Some(" [log10([Fe/H])]  metallicity                                       ")),
            KeywordRecord::new(Keyword::EBMINUSV,
                               Value::Raw(" "),
                               Option::Some(" [mag] E(B-V) reddening                                             ")),
            KeywordRecord::new(Keyword::AV,
                               Value::Raw(" "),
                               Option::Some(" [mag] A_v extinction                                               ")),
            KeywordRecord::new(Keyword::RADIUS,
                               Value::Raw(" "),
                               Option::Some(" [solar radii] stellar radius                                       ")),
            KeywordRecord::new(Keyword::TMINDEX,
                               Value::Raw(" "),
                               Option::Some(" unique 2MASS catalog ID                                            ")),
            KeywordRecord::new(Keyword::CHECKSUM,
                               Value::CharacterString("7k7A7h637h697h69"),
                               Option::Some(" HDU checksum updated 2017-03-08T02:47:56       ")),
            KeywordRecord::new(Keyword::DATASUM,
                               Value::CharacterString("0       "),
                               Option::Some(" data unit checksum updated 2017-03-08T02:47:56 ")),
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
                    Option::Some(" string version of target id                    ")
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
