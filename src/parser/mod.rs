//! The parser module is responsible for parsing FITS files.

use std::str;
use std::str::FromStr;
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
       map!(
           map_res!(
               is_not!("/"), // TODO Differentiate on the possible value types
               str::from_utf8
           ),
           Value::Raw
       )
);

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

named!(comment<&[u8], &str>,
       map_res!(
           do_parse!(
               tag!("/") >>
                   comment: take_while!(is_comment_character) >>
                   (comment)
           ),
           str::from_utf8
       ));

fn is_comment_character(chr: u8) -> bool {
    is_restricted_ascii(chr)
}

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
    use super::{fits, primary_header, keyword_record, keyword, valuecomment, character_string, end_record, blank_record};

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
            convenient(Keyword::SIMPLE, "                   T ",
                               Option::Some(" conforms to FITS standards                     ")),
            convenient(Keyword::BITPIX, "                   8 ",
                               Option::Some(" array data type                                ")),
            convenient(Keyword::NAXIS, "                   0 ",
                               Option::Some(" number of array dimensions                     ")),
            convenient(Keyword::EXTEND, "                   T ",
                               Option::Some(" file contains extensions                       ")),
            convenient(Keyword::NEXTEND, "                   2 ",
                               Option::Some(" number of standard extensions                  ")),
            convenient(Keyword::EXTNAME, "'PRIMARY '           ",
                               Option::Some(" name of extension                              ")),
            convenient(Keyword::EXTVER, "                   1 ",
                               Option::Some(" extension version number (not format version)  ")),
            convenient(Keyword::ORIGIN, "'Unofficial data product' ",
                               Option::Some(" institution responsible for creating this ")),
            convenient(Keyword::DATE, "'2017-03-08'         ",
                               Option::Some(" file creation date.                            ")),
            convenient(Keyword::CREATOR, "'kadenza '           ",
                               Option::Some(" pipeline job and program u                     ")),
            convenient(Keyword::PROCVER, "'2.1.dev '           ",
                               Option::Some(" SW version                                     ")),
            convenient(Keyword::FILEVER, "'0.0     '           ",
                               Option::Some(" file format version                            ")),
            convenient(Keyword::TIMVERSN, "'' ",
                               Option::Some(" OGIP memo number for file format                                 ")),
            convenient(Keyword::TELESCOP, "'Kepler  '           ",
                               Option::Some(" telescope                                      ")),
            convenient(Keyword::INSTRUME, "'Kepler Photometer'  ",
                               Option::Some(" detector type                                  ")),
            convenient(Keyword::OBJECT, "'EPIC 200164267'     ",
                               Option::Some(" string version of target id                    ")),
            convenient(Keyword::KEPLERID, "           200164267 ",
                               Option::Some(" unique Kepler target identifier                ")),
            convenient(Keyword::CHANNEL, "                  68 ",
                               Option::Some(" CCD channel                                    ")),
            convenient(Keyword::MODULE, "                  19 ",
                               Option::Some(" CCD module                                     ")),
            convenient(Keyword::OUTPUT, "                   4 ",
                               Option::Some(" CCD output                                     ")),
            convenient(Keyword::CAMPAIGN, "'' ",
                               Option::Some(" Observing campaign number                                        ")),
            convenient(Keyword::DATA_REL, "'' ",
                               Option::Some(" data release version number                                      ")),
            convenient(Keyword::OBSMODE, "'long cadence'       ",
                               Option::Some(" observing mode                                 ")),
            convenient(Keyword::MISSION, "'K2      '           ",
                               Option::Some(" Mission name                                   ")),
            convenient(Keyword::TTABLEID, "'' ",
                               Option::Some(" target table id                                                  ")),
            convenient(Keyword::RADESYS, "'ICRS    '           ",
                               Option::Some(" reference frame of celestial coordinates       ")),
            convenient(Keyword::RA_OBJ, "'' ",
                               Option::Some(" [deg] right ascension                                            ")),
            convenient(Keyword::DEC_OBJ, "'' ",
                               Option::Some(" [deg] declination                                                ")),
            convenient(Keyword::EQUINOX, "              2000.0 ",
                               Option::Some(" equinox of celestial coordinate system         ")),
            convenient(Keyword::PMRA, " ",
                               Option::Some(" [arcsec/yr] RA proper motion                                       ")),
            convenient(Keyword::PMDEC, " ",
                               Option::Some(" [arcsec/yr] Dec proper motion                                      ")),
            convenient(Keyword::PMTOTAL, " ",
                               Option::Some(" [arcsec/yr] total proper motion                                    ")),
            convenient(Keyword::PARALLAX, " ",
                               Option::Some(" [arcsec] parallax                                                  ")),
            convenient(Keyword::GLON, " ",
                               Option::Some(" [deg] galactic longitude                                           ")),
            convenient(Keyword::GLAT, " ",
                               Option::Some(" [deg] galactic latitude                                            ")),
            convenient(Keyword::GMAG, " ",
                               Option::Some(" [mag] SDSS g band magnitude                                        ")),
            convenient(Keyword::RMAG, " ",
                               Option::Some(" [mag] SDSS r band magnitude                                        ")),
            convenient(Keyword::IMAG, " ",
                               Option::Some(" [mag] SDSS i band magnitude                                        ")),
            convenient(Keyword::ZMAG, " ",
                               Option::Some(" [mag] SDSS z band magnitude                                        ")),
            convenient(Keyword::JMAG, " ",
                               Option::Some(" [mag] J band magnitude from 2MASS                                  ")),
            convenient(Keyword::HMAG, " ",
                               Option::Some(" [mag] H band magnitude from 2MASS                                  ")),
            convenient(Keyword::KMAG, " ",
                               Option::Some(" [mag] K band magnitude from 2MASS                                  ")),
            convenient(Keyword::KEPMAG, " ",
                               Option::Some(" [mag] Kepler magnitude (Kp)                                        ")),
            convenient(Keyword::GRCOLOR, " ",
                               Option::Some(" [mag] (g-r) color, SDSS bands                                      ")),
            convenient(Keyword::JKCOLOR, " ",
                               Option::Some(" [mag] (J-K) color, 2MASS bands                                     ")),
            convenient(Keyword::GKCOLOR, " ",
                               Option::Some(" [mag] (g-K) color, SDSS g - 2MASS K                                ")),
            convenient(Keyword::TEFF, " ",
                               Option::Some(" [K] Effective temperature                                          ")),
            convenient(Keyword::LOGG, " ",
                               Option::Some(" [cm/s2] log10 surface gravity                                      ")),
            convenient(Keyword::FEH, " ",
                               Option::Some(" [log10([Fe/H])]  metallicity                                       ")),
            convenient(Keyword::EBMINUSV, " ",
                               Option::Some(" [mag] E(B-V) reddening                                             ")),
            convenient(Keyword::AV, " ",
                               Option::Some(" [mag] A_v extinction                                               ")),
            convenient(Keyword::RADIUS, " ",
                               Option::Some(" [solar radii] stellar radius                                       ")),
            convenient(Keyword::TMINDEX, " ",
                               Option::Some(" unique 2MASS catalog ID                                            ")),
            convenient(Keyword::CHECKSUM, "'7k7A7h637h697h69'   ",
                               Option::Some(" HDU checksum updated 2017-03-08T02:47:56       ")),
            convenient(Keyword::DATASUM, "'0       '           ",
                               Option::Some(" data unit checksum updated 2017-03-08T02:47:56 ")),
        ))
    }

    fn convenient<'a>(keyword: Keyword, value: &'a str, comment: Option<&'a str>) -> KeywordRecord<'a> {
        KeywordRecord::new(keyword, Value::Raw(value), comment)
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
                    Value::Raw("'EPIC 200164267'     "),
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
                assert_eq!(value, Value::Raw("'EPIC 200164267'     "));
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
