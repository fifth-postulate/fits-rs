//! The parser module is responsible for parsing FITS files.

use std::str;
use std::str::FromStr;
use super::types::Keyword;

named!(fits<&[u8], ((Vec<Keyword>, (&[u8], Vec<&[u8]>), Vec<Vec<&[u8]> >), Vec<&[u8]>) >,
       pair!(primary_header, many0!( take!(2880) )));

named!(primary_header<&[u8], (Vec<Keyword>, (&[u8], Vec<&[u8]>), Vec<Vec<&[u8]> >)>,
       tuple!(
           many0!(keyword_record),
           end_record,
           many0!(blank_record)
       ));

named!(keyword_record<&[u8], Keyword>,
       map!( // TODO should use map_res!
           tuple!(
               take!(8),
               tag!("="),
               take!(71)
           ), |(slice, _, _) : (&[u8], &[u8], &[u8])| {
               match str::from_utf8(slice) {
                   Ok(s) => {
                       match Keyword::from_str(s) {
                           Ok(keyword) => keyword,
                           Err(_) => Keyword::SIMPLE // TODO this is not correct
                       }
                   }
                   Err(_) => Keyword::SIMPLE // TODO this is not correct
               }
           }));

named!(end_record<&[u8],(&[u8], Vec<&[u8]>)>, pair!(tag!("END"), count!(tag!(" "), 77)));

named!(blank_record<&[u8],Vec<&[u8]> >, count!(tag!(" "), 80));

#[cfg(test)]
mod tests {
    use nom::{IResult};
    use super::{fits, primary_header, end_record, blank_record, keyword_record};

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
            IResult::Done(_, _) => assert!(true),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }

    #[test]
    fn keyword_record_should_parse_a_keyword_record(){
        let data = "OBJECT  = 'EPIC 200164267'     / string version of target id                    "
            .as_bytes();

        let result = keyword_record(data);

        match result {
            IResult::Done(_,_) => assert!(true),
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
            IResult::Done(_,_) => assert!(true),
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
            IResult::Done(_,_) => assert!(true),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }
}
