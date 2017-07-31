#[macro_use]
extern crate nom;

use nom::{IResult, Needed, alphanumeric, is_alphanumeric};

named!(certain_length<&[u8], &[u8]>, take!(10));

named!(value<&[u8], &[u8]>,
       take_while!(is_alphanumeric));

named!(comment<&[u8], &[u8]>,
      do_parse!(
          tag!("/") >>
              comment: alphanumeric >>
              (comment)
      ));

named!(record<&[u8], (&[u8], Option<&[u8]>)>,
       do_parse!(
           v: value >>
               c: opt!(comment) >>
               ((v, c))
       ));

#[test]
fn certain_length_test() {
    assert_eq!(certain_length(&b"0123456789"[..]), IResult::Done(&b""[..], &b"0123456789"[..]));
    assert_eq!(certain_length(&b"012345678"[..]), IResult::Incomplete(Needed::Size(10)));
    assert_eq!(certain_length(&b"abcdefghijk"[..]), IResult::Done(&b"k"[..], &b"abcdefghij"[..]));
}

#[test]
fn value_test() {
    assert_eq!(value(&b""[..]),
               IResult::Done(&b""[..], &b""[..]));
    assert_eq!(value(&b"a"[..]),
               IResult::Done(&b""[..], &b"a"[..]));
    assert_eq!(value(&b"ab"[..]),
               IResult::Done(&b""[..], &b"ab"[..]));
    assert_eq!(value(&b"abc"[..]),
               IResult::Done(&b""[..], &b"abc"[..]));
}

#[test]
fn comment_test() {
    assert_eq!(comment(&b"/a"[..]), IResult::Done(&b""[..], &b"a"[..]));
    assert_eq!(comment(&b"/ab"[..]), IResult::Done(&b""[..], &b"ab"[..]));
    assert_eq!(comment(&b"/abc"[..]), IResult::Done(&b""[..], &b"abc"[..]));
}

#[test]
fn record_test() {
    assert_eq!(record(&b"abcd/123"[..]),
               IResult::Done(&b""[..], (&b"abcd"[..], Option::Some(&b"123"[..]))));
    assert_eq!(record(&b"abcd/123  "[..]),
               IResult::Done(&b"  "[..], (&b"abcd"[..], Option::Some(&b"123"[..]))));
    assert_eq!(record(&b"abcd  "[..]),
               IResult::Done(&b"  "[..], (&b"abcd"[..], Option::None)));
}
