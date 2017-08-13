//! The types modules describes all the structures to express FITS files.

use std::str::FromStr;
use std::fmt::{Display, Formatter, Error};

/// Representation of a FITS file.
#[derive(Debug, PartialEq)]
pub struct Fits<'a> {
    /// The primary HDU
    pub primary_hdu: HDU<'a>,
    /// The extention HDUs
    pub extensions: Vec<HDU<'a>>,
}

impl<'a> Fits<'a> {
    /// Create a Fits structure with a given primary header
    pub fn new(primary_hdu: HDU<'a>, extensions: Vec<HDU<'a>>) -> Fits<'a> {
        Fits {
            primary_hdu: primary_hdu,
            extensions: extensions,
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

    /// Determines the size in bits of the data array following this header.
    pub fn data_array_size(&self) -> usize {
        if self.is_primary() {
            lmle(self.primary_data_array_size(), 2880*8)
        } else {
            lmle(self.extention_data_array_size(), 2880*8)
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

    fn primary_data_array_size(&self) -> usize {
        (self.integer_value_of(&Keyword::BITPIX).unwrap_or(0i64).abs() * self.naxis_product()) as usize
    }

    fn extention_data_array_size(&self) -> usize {
        (self.integer_value_of(&Keyword::BITPIX).unwrap_or(0i64).abs() *
         self.integer_value_of(&Keyword::GCOUNT).unwrap_or(1i64) *
         (self.integer_value_of(&Keyword::PCOUNT).unwrap_or(0i64) + self.naxis_product())) as usize
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
    TDIMn(u16),
    TDISPn(u16),
    TEFF,
    TELESCOP,
    TFIELDS,
    TFORMn(u16),
    TIMVERSN,
    THEAP,
    TMINDEX,
    TNULLn(u16),
    TSCALn(u16),
    TTABLEID,
    TTYPEn(u16),
    TUNITn(u16),
    TZEROn(u16),
    XTENSION,
    ZMAG,
    Unprocessed, // TODO Remove the unprocessed keyword
}

/// Problems that could occur when parsing a `str` for a Keyword are enumerated here.
#[derive(Debug)]
pub enum ParseKeywordError {
    /// When a str can not be recognized as a keyword, this error will be returned.
    UnknownKeyword,
    /// When `NAXIS<number>` et. al. are parsed where `<number>` is not an actual number.
    NotANumber,
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
            "TFIELDS" => Ok(Keyword::TFIELDS),
            "THEAP" => Ok(Keyword::THEAP),
            "TIMVERSN" => Ok(Keyword::TIMVERSN),
            "TMINDEX" => Ok(Keyword::TMINDEX),
            "TTABLEID" => Ok(Keyword::TTABLEID),
            "XTENSION" => Ok(Keyword::XTENSION),
            "ZMAG" => Ok(Keyword::ZMAG),
            input @ _ => {
                if input.starts_with("TDIM") { // TODO refactor repetition
                    let (_, representation) = input.split_at(4);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::TDIMn(n)),
                        Err(_) => Err(ParseKeywordError::NotANumber)
                    }
                } else if input.starts_with("TDISP") {
                    let (_, representation) = input.split_at(5);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::TDISPn(n)),
                        Err(_) => Err(ParseKeywordError::NotANumber)
                    }
                } else if input.starts_with("TFORM") {
                    let (_, representation) = input.split_at(5);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::TFORMn(n)),
                        Err(_) => Err(ParseKeywordError::NotANumber)
                    }
                } else if input.starts_with("NAXIS") {
                    let (_, representation) = input.split_at(5);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::NAXISn(n)),
                        Err(_) => Err(ParseKeywordError::NotANumber)
                    }
                } else if input.starts_with("TNULL") {
                    let (_, representation) = input.split_at(5);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::TNULLn(n)),
                        Err(_) => Err(ParseKeywordError::NotANumber)
                    }
                } else if input.starts_with("TSCAL") {
                    let (_, representation) = input.split_at(5);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::TSCALn(n)),
                        Err(_) => Err(ParseKeywordError::NotANumber)
                    }
                } else if input.starts_with("TTYPE") {
                    let (_, representation) = input.split_at(5);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::TTYPEn(n)),
                        Err(_) => Err(ParseKeywordError::NotANumber)
                    }
                }  else if input.starts_with("TUNIT") {
                    let (_, representation) = input.split_at(5);
                    match u16::from_str(representation) {
                        Ok(n) => Ok(Keyword::TUNITn(n)),
                        Err(_) => Err(ParseKeywordError::NotANumber)
                    }
                } else {
                    let constructor = Keyword::TZEROn;
                    let tuples = vec!(("TZERO", &constructor));
                    let special_cases: Vec<PrefixedKeyword> =
                        tuples
                        .into_iter()
                        .map(|(prefix, constructor)|{ PrefixedKeyword::new(prefix, constructor)})
                        .collect();
                   for special_case in special_cases {
                        if special_case.handles(input) {
                            return special_case.transform(input)
                        }
                    }
                    Ok(Keyword::Unprocessed)
                    //Err(ParseKeywordError::UnknownKeyword)
                }
            }
        }
    }
}

trait KeywordSpecialCase {
    fn handles(&self, input: &str) -> bool;
    fn transform(&self, input: &str) -> Result<Keyword, ParseKeywordError>;
}

struct PrefixedKeyword<'a> {
    prefix: &'a str,
    constructor: &'a (Fn(u16) -> Keyword),
}

impl<'a> PrefixedKeyword<'a> {
    fn new(prefix: &'a str, constructor: &'a (Fn(u16) -> Keyword)) -> PrefixedKeyword<'a> {
        PrefixedKeyword { prefix: prefix, constructor: constructor }
    }
}

impl<'a> KeywordSpecialCase for PrefixedKeyword<'a> {
    fn handles(&self, input: &str) -> bool {
        input.starts_with(self.prefix)
    }

    fn transform(&self, input: &str) -> Result<Keyword, ParseKeywordError> {
        let (_, representation) = input.split_at(self.prefix.len());
        match u16::from_str(representation) {
            Ok(n) => Ok((self.constructor)(n)),
            Err(_) => Err(ParseKeywordError::NotANumber)
        }
    }
}

/// For input n and k, finds the least multiple of k such that n <= q*k and
/// (q-1)*k < n
fn lmle(n: usize, k: usize) -> usize {
    let (q, r) = (n / k, n % k);
    if r == 0 {
        q * k
    } else {
        (q + 1) * k
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
                extensions: vec!(),
            },
            Fits::new(HDU::new(Header::new(vec!())), vec!())
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
            ("TFIELDS", Keyword::TFIELDS),
            ("TIMVERSN", Keyword::TIMVERSN),
            ("THEAP", Keyword::THEAP),
            ("TMINDEX", Keyword::TMINDEX),
            ("TTABLEID", Keyword::TTABLEID),
            ("XTENSION", Keyword::XTENSION),
            ("ZMAG", Keyword::ZMAG),
        );

        for (input, expected) in data {
            assert_eq!(Keyword::from_str(input).unwrap(), expected);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn TDIMn_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::TDIMn(n);
            let representation = format!("TDIM{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn TDISPn_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::TDISPn(n);
            let representation = format!("TDISP{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
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

    #[allow(non_snake_case)]
    #[test]
    fn TFORM_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::TFORMn(n);
            let representation = format!("TFORM{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn TTYPE_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::TTYPEn(n);
            let representation = format!("TTYPE{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
        }
    }


    #[allow(non_snake_case)]
    #[test]
    fn TSCALn_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::TSCALn(n);
            let representation = format!("TSCAL{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn TZEROn_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::TZEROn(n);
            let representation = format!("TZERO{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn TNULL_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::TNULLn(n);
            let representation = format!("TNULL{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
        }
    }

    #[allow(non_snake_case)]
    #[test]
    fn TUNIT_should_be_parsed_from_str() {
        for n in 1u16..1000u16 {
            let keyword = Keyword::TUNITn(n);
            let representation = format!("TUNIT{}", n);

            assert_eq!(Keyword::from_str(&representation).unwrap(), keyword);
        }
    }

    #[test]
    fn should_also_parse_whitespace_keywords() {
        assert_eq!(Keyword::from_str("SIMPLE  ").unwrap(), Keyword::SIMPLE);
    }

    #[test]
    fn primary_header_should_determine_correct_data_array_size() {
        let header = Header::new(vec!(
            KeywordRecord::new(Keyword::SIMPLE, Value::Logical(true), Option::None),
            KeywordRecord::new(Keyword::BITPIX, Value::Integer(8i64), Option::None),
            KeywordRecord::new(Keyword::NAXIS, Value::Integer(2i64), Option::None),
            KeywordRecord::new(Keyword::NAXISn(1u16), Value::Integer(3i64), Option::None),
            KeywordRecord::new(Keyword::NAXISn(2u16), Value::Integer(5i64), Option::None),
            KeywordRecord::new(Keyword::END, Value::Undefined, Option::None),
        ));

        assert_eq!(header.data_array_size(), 1*(2880*8) as usize);
    }

    #[test]
    fn extension_header_should_determine_correct_data_array_size() {
        let header = Header::new(vec!(
            KeywordRecord::new(Keyword::XTENSION, Value::CharacterString("BINTABLE"), Option::None),
            KeywordRecord::new(Keyword::BITPIX, Value::Integer(128i64), Option::None),
            KeywordRecord::new(Keyword::NAXIS, Value::Integer(2i64), Option::None),
            KeywordRecord::new(Keyword::NAXISn(1u16), Value::Integer(3i64), Option::None),
            KeywordRecord::new(Keyword::NAXISn(2u16), Value::Integer(5i64), Option::None),
            KeywordRecord::new(Keyword::GCOUNT, Value::Integer(7i64), Option::None),
            KeywordRecord::new(Keyword::PCOUNT, Value::Integer(11i64), Option::None),
            KeywordRecord::new(Keyword::END, Value::Undefined, Option::None),
        ));

        assert_eq!(header.data_array_size(), 2*(2880*8) as usize);
    }
}
