//! Parser module is responsible for parsing FITS files.

named!(fits<&[u8], (Vec<&[u8]>, Vec<&[u8]>) >,
       pair!(many_m_n!(2, 2, take!(2880) ), many0!( take!(2880) )));

#[cfg(test)]
mod tests {
    use nom::{IResult};
    use super::{fits};

    #[test]
    fn it_should_parse_a_fits_file(){
        let data = include_bytes!("../../assets/images/k2-trappist1-unofficial-tpf-long-cadence.fits");

        let result = fits(data);

        match result {
            IResult::Done(_, (header, blocks)) => {
                assert_eq!(header.len(), 2);
                assert_eq!(blocks.len(), 3675);
            },
            IResult::Error(_) => panic!("Did not expect an error"),
            IResult::Incomplete(_) => panic!("Did not expect to be incomplete")
        }
    }
}
