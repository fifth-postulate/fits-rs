//! Parser module is responsible for parsing FITS files.

named!(fits<&[u8], Vec<&[u8]> >, many1!( take!(2880) ));

#[cfg(test)]
mod tests {
    use nom::{IResult};
    use super::{fits};

    #[test]
    fn it_should_parse_a_fits_file(){
        let data = include_bytes!("../../assets/images/k2-trappist1-unofficial-tpf-long-cadence.fits");

        let result = fits(data);

        match result {
            IResult::Done(_, blocks) => assert_eq!(blocks.len(), 3677),
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }
}
