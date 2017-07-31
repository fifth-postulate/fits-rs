//! The parser module is responsible for parsing FITS files.

use std::str;
use std::str::FromStr;
use super::types::{Fits, PrimaryHeader, KeywordRecord, Keyword, BlankRecord};

named!(fits<&[u8], Fits>,
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
               tag!("=") >>
               take!(71) >>
               (KeywordRecord::new(key))
       ));

named!(keyword<&[u8], Keyword>,
       map_res!(
           map_res!(
               take!(8),
               str::from_utf8),
           Keyword::from_str
       ));

named!(end_record<&[u8], Keyword>,
       map!(
           pair!(tag!("END"), count!(tag!(" "), 77)),
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
    use super::{fits, primary_header, keyword_record, keyword, end_record, blank_record};

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

    fn long_cadence_header() -> PrimaryHeader {
        PrimaryHeader::new(vec!(
            KeywordRecord::new(Keyword::SIMPLE),
            KeywordRecord::new(Keyword::BITPIX),
            KeywordRecord::new(Keyword::NAXIS),
            KeywordRecord::new(Keyword::EXTEND),
            KeywordRecord::new(Keyword::NEXTEND),
            KeywordRecord::new(Keyword::EXTNAME),
            KeywordRecord::new(Keyword::EXTVER),
            KeywordRecord::new(Keyword::ORIGIN),
            KeywordRecord::new(Keyword::DATE),
            KeywordRecord::new(Keyword::CREATOR),
            KeywordRecord::new(Keyword::PROCVER),
            KeywordRecord::new(Keyword::FILEVER),
            KeywordRecord::new(Keyword::TIMVERSN),
            KeywordRecord::new(Keyword::TELESCOP),
            KeywordRecord::new(Keyword::INSTRUME),
            KeywordRecord::new(Keyword::OBJECT),
            KeywordRecord::new(Keyword::KEPLERID),
            KeywordRecord::new(Keyword::CHANNEL),
            KeywordRecord::new(Keyword::MODULE),
            KeywordRecord::new(Keyword::OUTPUT),
            KeywordRecord::new(Keyword::CAMPAIGN),
            KeywordRecord::new(Keyword::DATA_REL),
            KeywordRecord::new(Keyword::OBSMODE),
            KeywordRecord::new(Keyword::MISSION),
            KeywordRecord::new(Keyword::TTABLEID),
            KeywordRecord::new(Keyword::RADESYS),
            KeywordRecord::new(Keyword::RA_OBJ),
            KeywordRecord::new(Keyword::DEC_OBJ),
            KeywordRecord::new(Keyword::EQUINOX),
            KeywordRecord::new(Keyword::PMRA),
            KeywordRecord::new(Keyword::PMDEC),
            KeywordRecord::new(Keyword::PMTOTAL),
            KeywordRecord::new(Keyword::PARALLAX),
            KeywordRecord::new(Keyword::GLON),
            KeywordRecord::new(Keyword::GLAT),
            KeywordRecord::new(Keyword::GMAG),
            KeywordRecord::new(Keyword::RMAG),
            KeywordRecord::new(Keyword::IMAG),
            KeywordRecord::new(Keyword::ZMAG),
            KeywordRecord::new(Keyword::JMAG),
            KeywordRecord::new(Keyword::HMAG),
            KeywordRecord::new(Keyword::KMAG),
            KeywordRecord::new(Keyword::KEPMAG),
            KeywordRecord::new(Keyword::GRCOLOR),
            KeywordRecord::new(Keyword::JKCOLOR),
            KeywordRecord::new(Keyword::GKCOLOR),
            KeywordRecord::new(Keyword::TEFF),
            KeywordRecord::new(Keyword::LOGG),
            KeywordRecord::new(Keyword::FEH),
            KeywordRecord::new(Keyword::EBMINUSV),
            KeywordRecord::new(Keyword::AV),
            KeywordRecord::new(Keyword::RADIUS),
            KeywordRecord::new(Keyword::TMINDEX),
            KeywordRecord::new(Keyword::CHECKSUM),
            KeywordRecord::new(Keyword::DATASUM),
        ))
    }

    #[test]
    fn keyword_record_should_parse_a_keyword_record(){
        let data = "OBJECT  = 'EPIC 200164267'     / string version of target id                    "
            .as_bytes();

        let result = keyword_record(data);

        match result {
            IResult::Done(_,k) => assert_eq!(k, KeywordRecord::new(Keyword::OBJECT)),
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
