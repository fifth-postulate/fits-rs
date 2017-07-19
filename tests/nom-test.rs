#[macro_use]
extern crate nom;

use nom::{IResult,digit};

use std::str;
use std::str::FromStr;

named!(number<i64>,
       map_res!(
           map_res!(
               ws!(digit),
               str::from_utf8),
           FromStr::from_str
       ));

#[test]
fn number_test() {
    assert_eq!(number(&b"     3"[..]), IResult::Done(&b""[..], 3));
    assert_eq!(number(&b"    37"[..]), IResult::Done(&b""[..], 37));
    assert_eq!(number(&b"   373"[..]), IResult::Done(&b""[..], 373));
    assert_eq!(number(&b"  3733"[..]), IResult::Done(&b""[..], 3733));
    assert_eq!(number(&b" 37337"[..]), IResult::Done(&b""[..], 37337));
    assert_eq!(number(&b"373379"[..]), IResult::Done(&b""[..], 373379));
}
