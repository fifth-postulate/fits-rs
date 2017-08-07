//! The types modules describes all the structures to express FITS files.

use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};

/// Representation of a FITS file.
#[derive(Debug, PartialEq)]
pub struct Fits<'a> {
    /// The primary HDU
    pub primary_hdu: HDU<'a>,
}

impl<'a> Fits<'a> {
    /// Create a Fits structure with a given primary header
    pub fn new(primary_hdu: HDU<'a>) -> Fits<'a> {
        Fits {
            primary_hdu: primary_hdu,
        }
    }
}

/// Header Data Unit, combination of a header and an optional data array.
#[derive(Debug, PartialEq)]
pub struct HDU<'a> {
    /// The header of this HDU.
    pub header: Header<'a>,
    /// The optional data array of this HDU.
    data_array: Option<DataArray>,
}

impl<'a> HDU<'a> {
    /// Create an HDU with a header, setting the data_array to none.
    pub fn new(header: Header<'a>) -> HDU<'a> {
        HDU { header: header, data_array: Option::None }
    }
}

/// The primary header of a FITS file.
#[derive(Debug, PartialEq)]
pub struct Header<'a> {
    /// The keyword records of the primary header.
    pub keyword_records: Vec<KeywordRecord<'a>>,
}

impl<'a> Header<'a> {
    /// Create a Header with a given set of keyword_records
    pub fn new(keyword_records: Vec<KeywordRecord<'a>>) -> Header<'a> {
        Header { keyword_records: keyword_records }
    }

    /// Determines the size of the data array following this header.
    pub fn data_array_size(&self) -> u64 {
        if self.is_primary() {
            self.primary_data_array_size()
        } else {
            self.extention_data_array_size()
        }
    }

    fn is_primary(&self) -> bool {
        self.has_keyword_record(&Keyword::SIMPLE)
    }

    fn has_keyword_record(&self, keyword: &Keyword) -> bool {
        for keyword_record in &self.keyword_records {
            if *keyword == keyword_record.keyword {
                return true
            }
        }
        false
    }

    fn primary_data_array_size(&self) -> u64 {
        (self.integer_value_of(&Keyword::BITPIX).unwrap_or(0i64).abs() * self.naxis_product()) as u64
    }

    fn extention_data_array_size(&self) -> u64 { // TODO correctly implement
        (self.integer_value_of(&Keyword::BITPIX).unwrap_or(0i64).abs() *
         self.integer_value_of(&Keyword::GCOUNT).unwrap_or(1i64) *
         (self.integer_value_of(&Keyword::PCOUNT).unwrap_or(0i64) + self.naxis_product())) as u64
    }

    fn integer_value_of(&self, keyword: &Keyword) -> Result<i64, ValueRetrievalError> {
        self.value_of(keyword).and_then(|value| {
            match value {
                Value::Integer(n) => Ok(n),
                _ => Err(ValueRetrievalError::NotAnInteger),
            }
        })
    }

    fn value_of(&self, keyword: &Keyword) -> Result<Value, ValueRetrievalError> {
        if self.has_keyword_record(&keyword) {
            for keyword_record in &self.keyword_records {
                if keyword_record.keyword == *keyword {
                    return Ok(keyword_record.value.clone())
                }
            }
        }
        Err(ValueRetrievalError::KeywordNotPresent)
    }

    fn naxis_product(&self) -> i64 {
        let limit = self.integer_value_of(&Keyword::NAXIS).unwrap_or(0i64);
        if limit > 0 {
            let mut product = 1i64;
            for n in 0..limit {
                let naxisn = Keyword::NAXISn((n + 1i64) as u16);
                product *= self.integer_value_of(&naxisn)
                    .expect(format!("NAXIS{} should be defined", n).as_str());
            }
            product
        } else {
            0i64
        }
    }
}

/// When asking for a value, these things can go wrong.
#[derive(Debug)]
pub enum ValueRetrievalError {
    /// The value associated with this keyword is not an integer.
    NotAnInteger,
    /// There is no value associated with this keyword.
    ValueUndefined,
    /// The keyword is not present in the header.
    KeywordNotPresent,
}

/// Placeholder for DataArray
#[derive(Debug, PartialEq)]
pub struct DataArray;

/// A keyword record contains information about a FITS header. It consists of a
/// keyword, the corresponding value and an optional comment.
#[derive(Debug, PartialEq)]
pub struct KeywordRecord<'a> {
    /// The keyword of this record.
    keyword: Keyword,
    /// The value of this record.
    value: Value<'a>,
    /// The comment of this record.
    comment: Option<&'a str>
}

impl<'a> KeywordRecord<'a> {
    /// Create a `KeywordRecord` from a specific `Keyword`.
    pub fn new(keyword: Keyword, value: Value<'a>, comment: Option<&'a str>) -> KeywordRecord<'a> {
        KeywordRecord { keyword: keyword, value: value, comment: comment }
    }
}

impl<'a> Display for KeywordRecord<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{:?}= {:?}/{}", self.keyword, self.value, self.comment.unwrap_or(""))
    }
}

/// The possible values of a KeywordRecord.
#[derive(Debug, PartialEq, Clone)]
pub enum Value<'a> {
    /// A string enclosed in single quotes `'`.
    CharacterString(&'a str),
    /// A logical constant signified by either an uppercase `F` or an uppercase `T`.
    Logical(bool),
    /// An optionally signed decimal integer.
    Integer(i64),
    /// Fixed format real floating point number.
    Real(f64),
    /// Complex number represented by a real and imaginary component.
    Complex((f64, f64)),
    /// When a value is not present
    Undefined,
}

/// A unit struct that will act as a placeholder for blank records.
#[derive(Debug, PartialEq)]
pub struct BlankRecord;

/// The various keywords that can be found in headers.
#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types, missing_docs)]
pub enum Keyword {
    AV,
    BITPIX,
    CAMPAIGN,
    CHANNEL,
    CHECKSUM,
    CREATOR,
    DATASUM,
    DATA_REL,
    DATE,
    DEC_OBJ,
    EBMINUSV,
    END,
    EQUINOX,
    EXTEND,
    EXTNAME,
    EXTVER,
    FEH,
    FILEVER,
    GCOUNT,
    GKCOLOR,
    GLAT,
    GLON,
    GMAG,
    GRCOLOR,
    HMAG,
    IMAG,
    INSTRUME,
    JKCOLOR,
    JMAG,
    KEPLERID,
    KEPMAG,
    KMAG,
    LOGG,
    MISSION,
    MODULE,
    NAXIS,
    NAXISn(u16),
    NEXTEND,
    OBJECT,
    OBSMODE,
    ORIGIN,
    OUTPUT,
    PARALLAX,
    PCOUNT,
    PMDEC,
    PMRA,
    PMTOTAL,
    PROCVER,
    RADESYS,
    RADIUS,
    RA_OBJ,
    RMAG,
    SIMPLE,
    TEFF,
    TELESCOP,
    TIMVERSN,
    TMINDEX,
    TTABLEID,
    ZMAG,
}

/// Problems that could occur when parsing a `str` for a Keyword are enumerated here.
#[derive(Debug)]
pub enum ParseKeywordError {
    /// When a str can not be recognized as a keyword, this error will be returned.
    UnknownKeyword,
    /// When `NAXIS<number>` where `<number>` is not an actual number.
    NotAnNaxisNumber,
}

impl FromStr for Keyword {
    type Err = ParseKeywordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim_right() {
            "AV" => Ok(Keyword::AV),
            "BITPIX" => Ok(Keyword::BITPIX),
            "CAMPAIGN" => Ok(Keyword::CAMPAIGN),
            "CHANNEL" => Ok(Keyword::CHANNEL),
            "CHECKSUM" => Ok(Keyword::CHECKSUM),
            "CREATOR" => Ok(Keyword::CREATOR),
            "DATASUM" => Ok(Keyword::DATASUM),
            "DATA_REL" => Ok(Keyword::DATA_REL),
            "DATE" => Ok(Keyword::DATE),
            "DEC_OBJ" => Ok(Keyword::DEC_OBJ),
            "EBMINUSV" => Ok(Keyword::EBMINUSV),
            "END" => Ok(Keyword::END),
            "EQUINOX" => Ok(Keyword::EQUINOX),
            "EXTEND" => Ok(Keyword::EXTEND),
            "EXTNAME" => Ok(Keyword::EXTNAME),
            "EXTVER" => Ok(Keyword::EXTVER),
            "FEH" => Ok(Keyword::FEH),
            "FILEVER" => Ok(Keyword::FILEVER),
            "GCOUNT" => Ok(Keyword::GCOUNT),
            "GKCOLOR" => Ok(Keyword::GKCOLOR),
            "GLAT" => Ok(Keyword::GLAT),
            "GLON" => Ok(Keyword::GLON),
            "GMAG" => Ok(Keyword::GMAG),
            "GRCOLOR" => Ok(Keyword::GRCOLOR),
            "HMAG" => Ok(Keyword::HMAG),
            "IMAG" => Ok(Keyword::IMAG),
            "INSTRUME" => Ok(Keyword::INSTRUME),
            "JKCOLOR" => Ok(Keyword::JKCOLOR),
            "JMAG" => Ok(Keyword::JMAG),
            "KEPLERID" => Ok(Keyword::KEPLERID),
            "KEPMAG" => Ok(Keyword::KEPMAG),
            "KMAG" => Ok(Keyword::KMAG),
            "LOGG" => Ok(Keyword::LOGG),
            "MISSION" => Ok(Keyword::MISSION),
            "MODULE" => Ok(Keyword::MODULE),
            "NAXIS" => Ok(Keyword::NAXIS),
            "NEXTEND" => Ok(Keyword::NEXTEND),
            "OBJECT" => Ok(Keyword::OBJECT),
            "OBSMODE" => Ok(Keyword::OBSMODE),
            "ORIGIN" => Ok(Keyword::ORIGIN),
            "OUTPUT" => Ok(Keyword::OUTPUT),
            "PARALLAX" => Ok(Keyword::PARALLAX),
            "PCOUNT" => Ok(Keyword::PCOUNT),
            "PMDEC" => Ok(Keyword::PMDEC),
            "PMRA" => Ok(Keyword::PMRA),
            "PMTOTAL" => Ok(Keyword::PMTOTAL),
            "PROCVER" => Ok(Keyword::PROCVER),
            "RADESYS" => Ok(Keyword::RADESYS),
            "RADIUS" => Ok(Keyword::RADIUS),
            "RA_OBJ" => Ok(Keyword::RA_OBJ),
            "RMAG" => Ok(Keyword::RMAG),
            "SIMPLE" => Ok(Keyword::SIMPLE),
            "TEFF" => Ok(Keyword::TEFF),
            "TELESCOP" => Ok(Keyword::TELESCOP),
            "TIMVERSN" => Ok(Keyword::TIMVERSN),
            "TMINDEX" => Ok(Keyword::TMINDEX),
            "TTABLEID" => Ok(Keyword::TTABLEID),
            "ZMAG" => Ok(Keyword::ZMAG),
            input @ _ => {
                if input.starts_with("NAXIS") {
                    let (_, representation) = input.split_at(5);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::NAXISn(n)),
                        Err(_) => Err(ParseKeywordError::NotAnNaxisNumber)
                    }
                } else {
                    Err(ParseKeywordError::UnknownKeyword)
                }
            }
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
            Fits {
                primary_hdu: HDU::new(Header::new(vec!())),
            },
            Fits::new(HDU::new(Header::new(vec!())))
        );
    }

    #[test]
    fn header_constructed_from_the_new_function_should_eq_hand_construction() {
        assert_eq!(
            Header { keyword_records: vec!(
                KeywordRecord::new(Keyword::SIMPLE, Value::Logical(true), Option::None),
                KeywordRecord::new(Keyword::NEXTEND, Value::Integer(0i64), Option::Some("no extensions")),
            )},
            Header::new(vec!(
                KeywordRecord::new(Keyword::SIMPLE, Value::Logical(true), Option::None),
                KeywordRecord::new(Keyword::NEXTEND, Value::Integer(0i64), Option::Some("no extensions")),
            ))
        );
    }

    #[test]
    fn keyword_record_constructed_from_the_new_function_should_eq_hand_construction() {
        assert_eq!(
            KeywordRecord { keyword: Keyword::ORIGIN, value: Value::Undefined, comment: Option::None },
            KeywordRecord::new(Keyword::ORIGIN, Value::Undefined, Option::None));
    }

    #[test]
    fn keywords_could_be_constructed_from_str() {
        let data = vec!(
            ("AV", Keyword::AV),
            ("BITPIX", Keyword::BITPIX),
            ("CAMPAIGN", Keyword::CAMPAIGN),
            ("CHANNEL", Keyword::CHANNEL),
            ("CHECKSUM", Keyword::CHECKSUM),
            ("CREATOR", Keyword::CREATOR),
            ("DATASUM", Keyword::DATASUM),
            ("DATA_REL", Keyword::DATA_REL),
            ("DATE", Keyword::DATE),
            ("DEC_OBJ", Keyword::DEC_OBJ),
            ("EBMINUSV", Keyword::EBMINUSV),
            ("END", Keyword::END),
            ("EQUINOX", Keyword::EQUINOX),
            ("EXTEND", Keyword::EXTEND),
            ("EXTVER", Keyword::EXTVER),
            ("FEH", Keyword::FEH),
            ("FILEVER", Keyword::FILEVER),
            ("GCOUNT", Keyword::GCOUNT),
            ("GKCOLOR", Keyword::GKCOLOR),
            ("GLAT", Keyword::GLAT),
            ("GLON", Keyword::GLON),
            ("GMAG", Keyword::GMAG),
            ("GRCOLOR", Keyword::GRCOLOR),
            ("HMAG", Keyword::HMAG),
            ("IMAG", Keyword::IMAG),
            ("INSTRUME", Keyword::INSTRUME),
            ("JKCOLOR", Keyword::JKCOLOR),
            ("JMAG", Keyword::JMAG),
            ("KEPLERID", Keyword::KEPLERID),
            ("KEPMAG", Keyword::KEPMAG),
            ("KMAG", Keyword::KMAG),
            ("LOGG", Keyword::LOGG),
            ("MISSION", Keyword::MISSION),
            ("MODULE", Keyword::MODULE),
            ("NAXIS", Keyword::NAXIS),
            ("NEXTEND", Keyword::NEXTEND),
            ("OBJECT", Keyword::OBJECT),
            ("OBSMODE", Keyword::OBSMODE),
            ("ORIGIN", Keyword::ORIGIN),
            ("OUTPUT", Keyword::OUTPUT),
            ("PARALLAX", Keyword::PARALLAX),
            ("PCOUNT", Keyword::PCOUNT),
            ("PMDEC", Keyword::PMDEC),
            ("PMRA", Keyword::PMRA),
            ("PMTOTAL", Keyword::PMTOTAL),
            ("PROCVER", Keyword::PROCVER),
            ("RADESYS", Keyword::RADESYS),
            ("RADIUS", Keyword::RADIUS),
            ("RA_OBJ", Keyword::RA_OBJ),
            ("RMAG", Keyword::RMAG),
            ("SIMPLE", Keyword::SIMPLE),
            ("TEFF", Keyword::TEFF),
            ("TELESCOP", Keyword::TELESCOP),
            ("TIMVERSN", Keyword::TIMVERSN),
            ("TMINDEX", Keyword::TMINDEX),
            ("TTABLEID", Keyword::TTABLEID),
            ("ZMAG", Keyword::ZMAG),
        );

        for (input, expected) in data {
            assert_eq!(Keyword::from_str(input).unwrap(), expected);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn NAXISn_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::NAXISn(n);
            let representation = format!("NAXIS{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
        }
    }

    #[test]
    fn should_also_parse_whitespace_keywords() {
        assert_eq!(Keyword::from_str("SIMPLE  ").unwrap(), Keyword::SIMPLE);
    }

    #[test]
    fn primary_header_should_determine_correct_size_primary_data_array() {
        let header = Header::new(vec!(
            KeywordRecord::new(Keyword::SIMPLE, Value::Logical(true), Option::None),
            KeywordRecord::new(Keyword::BITPIX, Value::Integer(8i64), Option::None),
            KeywordRecord::new(Keyword::NAXIS, Value::Integer(2i64), Option::None),
            KeywordRecord::new(Keyword::NAXISn(1u16), Value::Integer(3i64), Option::None),
            KeywordRecord::new(Keyword::NAXISn(2u16), Value::Integer(5i64), Option::None),
            KeywordRecord::new(Keyword::END, Value::Undefined, Option::None),
        ));

        assert_eq!(header.data_array_size(), 8*3*5);
    }
}
