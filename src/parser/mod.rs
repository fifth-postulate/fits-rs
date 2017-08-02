//! The parser module is responsible for parsing FITS files.

use std::str;
use std::str::FromStr;
use super::types::{Fits, PrimaryHeader, KeywordRecord, Keyword, BlankRecord};

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

named!(valuecomment<&[u8], (&str, Option<&str>)>,
       flat_map!(
           take!(70),
           pair!(
               value,
               opt!(comment)
           )));

named!(value<&[u8], &str>,
       map_res!(
           is_not!("/"), // TODO Differentiate on the possible value types
           str::from_utf8
       )
);

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
    use super::super::types::{Fits, PrimaryHeader, KeywordRecord, Keyword, BlankRecord};
    use super::{fits, primary_header, keyword_record, keyword, valuecomment, end_record, blank_record};

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
            KeywordRecord::new(Keyword::SIMPLE, "                   T ",
                               Option::Some(" conforms to FITS standards                     ")),
            KeywordRecord::new(Keyword::BITPIX, "                   8 ",
                               Option::Some(" array data type                                ")),
            KeywordRecord::new(Keyword::NAXIS, "                   0 ",
                               Option::Some(" number of array dimensions                     ")),
            KeywordRecord::new(Keyword::EXTEND, "                   T ",
                               Option::Some(" file contains extensions                       ")),
            KeywordRecord::new(Keyword::NEXTEND, "                   2 ",
                               Option::Some(" number of standard extensions                  ")),
            KeywordRecord::new(Keyword::EXTNAME, "'PRIMARY '           ",
                               Option::Some(" name of extension                              ")),
            KeywordRecord::new(Keyword::EXTVER, "                   1 ",
                               Option::Some(" extension version number (not format version)  ")),
            KeywordRecord::new(Keyword::ORIGIN, "'Unofficial data product' ",
                               Option::Some(" institution responsible for creating this ")),
            KeywordRecord::new(Keyword::DATE, "'2017-03-08'         ",
                               Option::Some(" file creation date.                            ")),
            KeywordRecord::new(Keyword::CREATOR, "'kadenza '           ",
                               Option::Some(" pipeline job and program u                     ")),
            KeywordRecord::new(Keyword::PROCVER, "'2.1.dev '           ",
                               Option::Some(" SW version                                     ")),
            KeywordRecord::new(Keyword::FILEVER, "'0.0     '           ",
                               Option::Some(" file format version                            ")),
            KeywordRecord::new(Keyword::TIMVERSN, "'' ",
                               Option::Some(" OGIP memo number for file format                                 ")),
            KeywordRecord::new(Keyword::TELESCOP, "'Kepler  '           ",
                               Option::Some(" telescope                                      ")),
            KeywordRecord::new(Keyword::INSTRUME, "'Kepler Photometer'  ",
                               Option::Some(" detector type                                  ")),
            KeywordRecord::new(Keyword::OBJECT, "'EPIC 200164267'     ",
                               Option::Some(" string version of target id                    ")),
            KeywordRecord::new(Keyword::KEPLERID, "           200164267 ",
                               Option::Some(" unique Kepler target identifier                ")),
            KeywordRecord::new(Keyword::CHANNEL, "                  68 ",
                               Option::Some(" CCD channel                                    ")),
            KeywordRecord::new(Keyword::MODULE, "                  19 ",
                               Option::Some(" CCD module                                     ")),
            KeywordRecord::new(Keyword::OUTPUT, "                   4 ",
                               Option::Some(" CCD output                                     ")),
            KeywordRecord::new(Keyword::CAMPAIGN, "'' ",
                               Option::Some(" Observing campaign number                                        ")),
            KeywordRecord::new(Keyword::DATA_REL, "'' ",
                               Option::Some(" data release version number                                      ")),
            KeywordRecord::new(Keyword::OBSMODE, "'long cadence'       ",
                               Option::Some(" observing mode                                 ")),
            KeywordRecord::new(Keyword::MISSION, "'K2      '           ",
                               Option::Some(" Mission name                                   ")),
            KeywordRecord::new(Keyword::TTABLEID, "'' ",
                               Option::Some(" target table id                                                  ")),
            KeywordRecord::new(Keyword::RADESYS, "'ICRS    '           ",
                               Option::Some(" reference frame of celestial coordinates       ")),
            KeywordRecord::new(Keyword::RA_OBJ, "'' ",
                               Option::Some(" [deg] right ascension                                            ")),
            KeywordRecord::new(Keyword::DEC_OBJ, "'' ",
                               Option::Some(" [deg] declination                                                ")),
            KeywordRecord::new(Keyword::EQUINOX, "              2000.0 ",
                               Option::Some(" equinox of celestial coordinate system         ")),
            KeywordRecord::new(Keyword::PMRA, " ",
                               Option::Some(" [arcsec/yr] RA proper motion                                       ")),
            KeywordRecord::new(Keyword::PMDEC, " ",
                               Option::Some(" [arcsec/yr] Dec proper motion                                      ")),
            KeywordRecord::new(Keyword::PMTOTAL, " ",
                               Option::Some(" [arcsec/yr] total proper motion                                    ")),
            KeywordRecord::new(Keyword::PARALLAX, " ",
                               Option::Some(" [arcsec] parallax                                                  ")),
            KeywordRecord::new(Keyword::GLON, " ",
                               Option::Some(" [deg] galactic longitude                                           ")),
            KeywordRecord::new(Keyword::GLAT, " ",
                               Option::Some(" [deg] galactic latitude                                            ")),
            KeywordRecord::new(Keyword::GMAG, " ",
                               Option::Some(" [mag] SDSS g band magnitude                                        ")),
            KeywordRecord::new(Keyword::RMAG, " ",
                               Option::Some(" [mag] SDSS r band magnitude                                        ")),
            KeywordRecord::new(Keyword::IMAG, " ",
                               Option::Some(" [mag] SDSS i band magnitude                                        ")),
            KeywordRecord::new(Keyword::ZMAG, " ",
                               Option::Some(" [mag] SDSS z band magnitude                                        ")),
            KeywordRecord::new(Keyword::JMAG, " ",
                               Option::Some(" [mag] J band magnitude from 2MASS                                  ")),
            KeywordRecord::new(Keyword::HMAG, " ",
                               Option::Some(" [mag] H band magnitude from 2MASS                                  ")),
            KeywordRecord::new(Keyword::KMAG, " ",
                               Option::Some(" [mag] K band magnitude from 2MASS                                  ")),
            KeywordRecord::new(Keyword::KEPMAG, " ",
                               Option::Some(" [mag] Kepler magnitude (Kp)                                        ")),
            KeywordRecord::new(Keyword::GRCOLOR, " ",
                               Option::Some(" [mag] (g-r) color, SDSS bands                                      ")),
            KeywordRecord::new(Keyword::JKCOLOR, " ",
                               Option::Some(" [mag] (J-K) color, 2MASS bands                                     ")),
            KeywordRecord::new(Keyword::GKCOLOR, " ",
                               Option::Some(" [mag] (g-K) color, SDSS g - 2MASS K                                ")),
            KeywordRecord::new(Keyword::TEFF, " ",
                               Option::Some(" [K] Effective temperature                                          ")),
            KeywordRecord::new(Keyword::LOGG, " ",
                               Option::Some(" [cm/s2] log10 surface gravity                                      ")),
            KeywordRecord::new(Keyword::FEH, " ",
                               Option::Some(" [log10([Fe/H])]  metallicity                                       ")),
            KeywordRecord::new(Keyword::EBMINUSV, " ",
                               Option::Some(" [mag] E(B-V) reddening                                             ")),
            KeywordRecord::new(Keyword::AV, " ",
                               Option::Some(" [mag] A_v extinction                                               ")),
            KeywordRecord::new(Keyword::RADIUS, " ",
                               Option::Some(" [solar radii] stellar radius                                       ")),
            KeywordRecord::new(Keyword::TMINDEX, " ",
                               Option::Some(" unique 2MASS catalog ID                                            ")),
            KeywordRecord::new(Keyword::CHECKSUM, "'7k7A7h637h697h69'   ",
                               Option::Some(" HDU checksum updated 2017-03-08T02:47:56       ")),
            KeywordRecord::new(Keyword::DATASUM, "'0       '           ",
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
                    "'EPIC 200164267'     ",
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
                assert_eq!(value, "'EPIC 200164267'     ");
                assert_eq!(comment, Option::Some(" string version of target id                    "));
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
