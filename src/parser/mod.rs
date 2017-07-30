//! The parser module is responsible for parsing FITS files.

use std::str;
use std::str::FromStr;
use super::types::{PrimaryHeader, KeywordRecord, Keyword, BlankRecord};

named!(fits<&[u8], (PrimaryHeader, Vec<&[u8]>) >,
       pair!(primary_header, many0!( take!(2880) )));

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
               (KeywordRecord::create(key))
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
    use super::super::types::{PrimaryHeader, KeywordRecord, Keyword, BlankRecord};
    use super::{fits, primary_header, keyword_record, keyword, end_record, blank_record};

    #[ignore]
    #[test]
    fn it_should_parse_a_fits_file(){
        let data = include_bytes!("../../assets/images/k2-trappist1-unofficial-tpf-long-cadence.fits");

        let result = fits(data);

        match result {
            IResult::Done(_, (header, blocks)) => {
                assert_eq!(blocks.len(), 3675);
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
            KeywordRecord::create(Keyword::SIMPLE),
            KeywordRecord::create(Keyword::BITPIX),
            KeywordRecord::create(Keyword::NAXIS),
            KeywordRecord::create(Keyword::EXTEND),
            KeywordRecord::create(Keyword::NEXTEND),
            KeywordRecord::create(Keyword::EXTNAME),
            KeywordRecord::create(Keyword::EXTVER),
            KeywordRecord::create(Keyword::ORIGIN),
            KeywordRecord::create(Keyword::DATE),
            KeywordRecord::create(Keyword::CREATOR),
            KeywordRecord::create(Keyword::PROCVER),
            KeywordRecord::create(Keyword::FILEVER),
            KeywordRecord::create(Keyword::TIMVERSN),
            KeywordRecord::create(Keyword::TELESCOP),
            KeywordRecord::create(Keyword::INSTRUME),
            KeywordRecord::create(Keyword::OBJECT),
            KeywordRecord::create(Keyword::KEPLERID),
            KeywordRecord::create(Keyword::CHANNEL),
            KeywordRecord::create(Keyword::MODULE),
            KeywordRecord::create(Keyword::OUTPUT),
            KeywordRecord::create(Keyword::CAMPAIGN),
            KeywordRecord::create(Keyword::DATA_REL),
            KeywordRecord::create(Keyword::OBSMODE),
            KeywordRecord::create(Keyword::MISSION),
            KeywordRecord::create(Keyword::TTABLEID),
            KeywordRecord::create(Keyword::RADESYS),
            KeywordRecord::create(Keyword::RA_OBJ),
            KeywordRecord::create(Keyword::DEC_OBJ),
            KeywordRecord::create(Keyword::EQUINOX),
            KeywordRecord::create(Keyword::PMRA),
            KeywordRecord::create(Keyword::PMDEC),
            KeywordRecord::create(Keyword::PMTOTAL),
            KeywordRecord::create(Keyword::PARALLAX),
            KeywordRecord::create(Keyword::GLON),
            KeywordRecord::create(Keyword::GLAT),
            KeywordRecord::create(Keyword::GMAG),
            KeywordRecord::create(Keyword::RMAG),
            KeywordRecord::create(Keyword::IMAG),
            KeywordRecord::create(Keyword::ZMAG),
            KeywordRecord::create(Keyword::JMAG),
            KeywordRecord::create(Keyword::HMAG),
            KeywordRecord::create(Keyword::KMAG),
            KeywordRecord::create(Keyword::KEPMAG),
            KeywordRecord::create(Keyword::GRCOLOR),
            KeywordRecord::create(Keyword::JKCOLOR),
            KeywordRecord::create(Keyword::GKCOLOR),
            KeywordRecord::create(Keyword::TEFF),
            KeywordRecord::create(Keyword::LOGG),
            KeywordRecord::create(Keyword::FEH),
            KeywordRecord::create(Keyword::EBMINUSV),
            KeywordRecord::create(Keyword::AV),
            KeywordRecord::create(Keyword::RADIUS),
            KeywordRecord::create(Keyword::TMINDEX),
            KeywordRecord::create(Keyword::CHECKSUM),
            KeywordRecord::create(Keyword::DATASUM),
        ))
    }

    #[test]
    fn keyword_record_should_parse_a_keyword_record(){
        let data = "OBJECT  = 'EPIC 200164267'     / string version of target id                    "
            .as_bytes();

        let result = keyword_record(data);

        match result {
            IResult::Done(_,k) => assert_eq!(k, KeywordRecord::create(Keyword::OBJECT)),
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
