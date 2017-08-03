//! The types modules describes all the structures to express FITS files.

use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};

/// Representation of a FITS file.
#[derive(Debug, PartialEq)]
pub struct Fits<'a> {
    /// The primary header
    pub primary_header: PrimaryHeader<'a>
}

impl<'a> Fits<'a> {
    /// Create a Fits structure with a given primary header
    pub fn new(primary_header: PrimaryHeader<'a>) -> Fits<'a> {
        Fits { primary_header: primary_header }
    }
}

/// The primary header of a FITS file.
#[derive(Debug, PartialEq)]
pub struct PrimaryHeader<'a> {
    /// The keyword records of the primary header.
    pub keyword_records: Vec<KeywordRecord<'a>>,
}

impl<'a> PrimaryHeader<'a> {
    /// Create a PrimaryHeader with a given set of keyword_records
    pub fn new(keyword_records: Vec<KeywordRecord<'a>>) -> PrimaryHeader<'a> {
        PrimaryHeader { keyword_records: keyword_records }
    }
}

/// A keyword record contains information about a FITS header. It consists of a
/// keyword, the corresponding value and an optional comment.
#[derive(Debug, PartialEq)]
pub struct KeywordRecord<'a> {
    /// The keyword of this record.
    keyword: Keyword,
    /// The value of this record.
    value: &'a str,
    /// The comment of this record.
    comment: Option<&'a str>
}

impl<'a> KeywordRecord<'a> {
    /// Create a `KeywordRecord` from a specific `Keyword`.
    pub fn new(keyword: Keyword, value: &'a str, comment: Option<&'a str>) -> KeywordRecord<'a> {
        KeywordRecord { keyword: keyword, value: value, comment: comment }
    }
}

impl<'a> Display for KeywordRecord<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}= {}/{}", self.keyword, self.value, self.comment.unwrap_or(""))
    }
}

/// A unit tuple that will act as a placeholder for blank records.
#[derive(Debug, PartialEq)]
pub struct BlankRecord;

/// The various keywords that can be found in headers.
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types, missing_docs)]
pub enum Keyword {
    SIMPLE,
    BITPIX,
    NAXIS,
    NAXISn(u8),
    EXTEND,
    NEXTEND,
    EXTNAME,
    EXTVER,
    ORIGIN,
    DATE,
    CREATOR,
    PROCVER,
    FILEVER,
    TIMVERSN,
    TELESCOP,
    INSTRUME,
    OBJECT,
    KEPLERID,
    CHANNEL,
    MODULE,
    OUTPUT,
    CAMPAIGN,
    DATA_REL,
    OBSMODE,
    MISSION,
    TTABLEID,
    RADESYS,
    RA_OBJ,
    DEC_OBJ,
    EQUINOX,
    PMRA,
    PMDEC,
    PMTOTAL,
    PARALLAX,
    GLON,
    GLAT,
    GMAG,
    RMAG,
    IMAG,
    ZMAG,
    JMAG,
    HMAG,
    KMAG,
    KEPMAG,
    GRCOLOR,
    JKCOLOR,
    GKCOLOR,
    TEFF,
    LOGG,
    FEH,
    EBMINUSV,
    AV,
    RADIUS,
    TMINDEX,
    CHECKSUM,
    DATASUM,
    END,
}

/// Problems that could occur when parsing a `str` are enumerated here.
#[derive(Debug)]
pub enum ParseKeywordError {
    /// When a str can not be recognized as a keyword, this error will be returned
    UnknownKeyword
}

impl FromStr for Keyword {
    type Err = ParseKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO parse NAXIS1 correctly
        match s.trim_right() {
            "SIMPLE" => Ok(Keyword::SIMPLE),
            "BITPIX" => Ok(Keyword::BITPIX),
            "NAXIS" => Ok(Keyword::NAXIS),
            "EXTEND" => Ok(Keyword::EXTEND),
            "NEXTEND" => Ok(Keyword::NEXTEND),
            "EXTNAME" => Ok(Keyword::EXTNAME),
            "EXTVER" => Ok(Keyword::EXTVER),
            "ORIGIN" => Ok(Keyword::ORIGIN),
            "DATE" => Ok(Keyword::DATE),
            "CREATOR" => Ok(Keyword::CREATOR),
            "PROCVER" => Ok(Keyword::PROCVER),
            "FILEVER" => Ok(Keyword::FILEVER),
            "TIMVERSN" => Ok(Keyword::TIMVERSN),
            "TELESCOP" => Ok(Keyword::TELESCOP),
            "INSTRUME" => Ok(Keyword::INSTRUME),
            "OBJECT" => Ok(Keyword::OBJECT),
            "KEPLERID" => Ok(Keyword::KEPLERID),
            "CHANNEL" => Ok(Keyword::CHANNEL),
            "MODULE" => Ok(Keyword::MODULE),
            "OUTPUT" => Ok(Keyword::OUTPUT),
            "CAMPAIGN" => Ok(Keyword::CAMPAIGN),
            "DATA_REL" => Ok(Keyword::DATA_REL),
            "OBSMODE" => Ok(Keyword::OBSMODE),
            "MISSION" => Ok(Keyword::MISSION),
            "TTABLEID" => Ok(Keyword::TTABLEID),
            "RADESYS" => Ok(Keyword::RADESYS),
            "RA_OBJ" => Ok(Keyword::RA_OBJ),
            "DEC_OBJ" => Ok(Keyword::DEC_OBJ),
            "EQUINOX" => Ok(Keyword::EQUINOX),
            "PMRA" => Ok(Keyword::PMRA),
            "PMDEC" => Ok(Keyword::PMDEC),
            "PMTOTAL" => Ok(Keyword::PMTOTAL),
            "PARALLAX" => Ok(Keyword::PARALLAX),
            "GLON" => Ok(Keyword::GLON),
            "GLAT" => Ok(Keyword::GLAT),
            "GMAG" => Ok(Keyword::GMAG),
            "RMAG" => Ok(Keyword::RMAG),
            "IMAG" => Ok(Keyword::IMAG),
            "ZMAG" => Ok(Keyword::ZMAG),
            "JMAG" => Ok(Keyword::JMAG),
            "HMAG" => Ok(Keyword::HMAG),
            "KMAG" => Ok(Keyword::KMAG),
            "KEPMAG" => Ok(Keyword::KEPMAG),
            "GRCOLOR" => Ok(Keyword::GRCOLOR),
            "JKCOLOR" => Ok(Keyword::JKCOLOR),
            "GKCOLOR" => Ok(Keyword::GKCOLOR),
            "TEFF" => Ok(Keyword::TEFF),
            "LOGG" => Ok(Keyword::LOGG),
            "FEH" => Ok(Keyword::FEH),
            "EBMINUSV" => Ok(Keyword::EBMINUSV),
            "AV" => Ok(Keyword::AV),
            "RADIUS" => Ok(Keyword::RADIUS),
            "TMINDEX" => Ok(Keyword::TMINDEX),
            "CHECKSUM" => Ok(Keyword::CHECKSUM),
            "DATASUM" => Ok(Keyword::DATASUM),
            "END" => Ok(Keyword::END),
            _ => Err(ParseKeywordError::UnknownKeyword)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn fits_constructed_from_the_new_function_should_eq_hand_construction() {
        assert_eq!(
            Fits { primary_header: PrimaryHeader::new(vec!()) },
            Fits::new(PrimaryHeader::new(vec!()))
        );
    }

    #[test]
    fn primary_header_constructed_from_the_new_function_shoul_eq_hand_construction() {
        assert_eq!(
            PrimaryHeader { keyword_records: vec!(
                KeywordRecord::new(Keyword::SIMPLE, "T", Option::None),
                KeywordRecord::new(Keyword::NEXTEND, "0", Option::Some("no extensions")),
            )},
            PrimaryHeader::new(vec!(
                KeywordRecord::new(Keyword::SIMPLE, "T", Option::None),
                KeywordRecord::new(Keyword::NEXTEND, "0", Option::Some("no extensions")),
            ))
        );
    }

    #[test]
    fn keyword_record_constructed_from_the_new_function_should_eq_hand_construction() {
        assert_eq!(
            KeywordRecord { keyword: Keyword::ORIGIN, value: "", comment: Option::None },
            KeywordRecord::new(Keyword::ORIGIN, "", Option::None));
    }

    #[test]
    fn keywords_could_be_constructed_from_str() {
        let data = vec!(
            ("SIMPLE", Keyword::SIMPLE),
            ("BITPIX", Keyword::BITPIX),
            ("NAXIS", Keyword::NAXIS),
            ("EXTEND", Keyword::EXTEND),
            ("NEXTEND", Keyword::NEXTEND),
            ("EXTVER", Keyword::EXTVER),
            ("ORIGIN", Keyword::ORIGIN),
            ("DATE", Keyword::DATE),
            ("CREATOR", Keyword::CREATOR),
            ("PROCVER", Keyword::PROCVER),
            ("FILEVER", Keyword::FILEVER),
            ("TIMVERSN", Keyword::TIMVERSN),
            ("TELESCOP", Keyword::TELESCOP),
            ("INSTRUME", Keyword::INSTRUME),
            ("OBJECT", Keyword::OBJECT),
            ("KEPLERID", Keyword::KEPLERID),
            ("CHANNEL", Keyword::CHANNEL),
            ("MODULE", Keyword::MODULE),
            ("OUTPUT", Keyword::OUTPUT),
            ("CAMPAIGN", Keyword::CAMPAIGN),
            ("DATA_REL", Keyword::DATA_REL),
            ("OBSMODE", Keyword::OBSMODE),
            ("MISSION", Keyword::MISSION),
            ("TTABLEID", Keyword::TTABLEID),
            ("RADESYS", Keyword::RADESYS),
            ("RA_OBJ", Keyword::RA_OBJ),
            ("DEC_OBJ", Keyword::DEC_OBJ),
            ("EQUINOX", Keyword::EQUINOX),
            ("PMRA", Keyword::PMRA),
            ("PMDEC", Keyword::PMDEC),
            ("PMTOTAL", Keyword::PMTOTAL),
            ("PARALLAX", Keyword::PARALLAX),
            ("GLON", Keyword::GLON),
            ("GLAT", Keyword::GLAT),
            ("GMAG", Keyword::GMAG),
            ("RMAG", Keyword::RMAG),
            ("IMAG", Keyword::IMAG),
            ("ZMAG", Keyword::ZMAG),
            ("JMAG", Keyword::JMAG),
            ("HMAG", Keyword::HMAG),
            ("KMAG", Keyword::KMAG),
            ("KEPMAG", Keyword::KEPMAG),
            ("GRCOLOR", Keyword::GRCOLOR),
            ("JKCOLOR", Keyword::JKCOLOR),
            ("GKCOLOR", Keyword::GKCOLOR),
            ("TEFF", Keyword::TEFF),
            ("LOGG", Keyword::LOGG),
            ("FEH", Keyword::FEH),
            ("EBMINUSV", Keyword::EBMINUSV),
            ("AV", Keyword::AV),
            ("RADIUS", Keyword::RADIUS),
            ("TMINDEX", Keyword::TMINDEX),
            ("CHECKSUM", Keyword::CHECKSUM),
            ("DATASUM", Keyword::DATASUM),
            ("END", Keyword::END),
        );

        for (input, expected) in data {
            assert_eq!(Keyword::from_str(input).unwrap(), expected);
        }
    }

    #[test]
    fn should_also_parse_whitespace_keywords() {
        assert_eq!(Keyword::from_str("SIMPLE  ").unwrap(), Keyword::SIMPLE);
    }
}
