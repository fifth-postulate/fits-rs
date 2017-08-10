#[macro_use]
extern crate nom;

use nom::{IResult, digit, alpha};

named!(without_complete<&[u8], (&[u8], Option<&[u8]>)>,
       do_parse!(
           first: digit >>
               second: opt!(alpha) >>
               (first, second)
       ));

named!(with_complete<&[u8], (&[u8], Option<&[u8]>)>,
       do_parse!(
           first: digit >>
               second: opt!(complete!(alpha)) >>
               (first, second)
       ));

#[test]
fn without_complete_should_parse_digit_alpha() {
    assert_eq!(without_complete(&b"1a"[..]), IResult::Done(&b""[..], (&b"1"[..], Option::Some(&b"a"[..]))));
    assert_eq!(without_complete(&b"12a"[..]), IResult::Done(&b""[..], (&b"12"[..], Option::Some(&b"a"[..]))));
    assert_eq!(without_complete(&b"1ab"[..]), IResult::Done(&b""[..], (&b"1"[..], Option::Some(&b"ab"[..]))));
    assert_eq!(without_complete(&b"12ab"[..]), IResult::Done(&b""[..], (&b"12"[..], Option::Some(&b"ab"[..]))));
}

#[ignore]
#[allow(non_snake_case)]
#[test]
fn without_complete_should_parse_digit_TEST_SHOULD_BE_IGNORED() {
    assert_eq!(without_complete(&b"1"[..]), IResult::Done(&b""[..], (&b"1"[..], Option::None)));
    assert_eq!(without_complete(&b"12"[..]), IResult::Done(&b""[..], (&b"12"[..], Option::None)));
}

#[test]
fn with_complete_should_parse_digit_alpha() {
    assert_eq!(with_complete(&b"1a"[..]), IResult::Done(&b""[..], (&b"1"[..], Option::Some(&b"a"[..]))));
    assert_eq!(with_complete(&b"12a"[..]), IResult::Done(&b""[..], (&b"12"[..], Option::Some(&b"a"[..]))));
    assert_eq!(with_complete(&b"1ab"[..]), IResult::Done(&b""[..], (&b"1"[..], Option::Some(&b"ab"[..]))));
    assert_eq!(with_complete(&b"12ab"[..]), IResult::Done(&b""[..], (&b"12"[..], Option::Some(&b"ab"[..]))));
}

#[test]
fn with_complete_should_parse_digit() {
    assert_eq!(with_complete(&b"1"[..]), IResult::Done(&b""[..], (&b"1"[..], Option::None)));
    assert_eq!(with_complete(&b"12"[..]), IResult::Done(&b""[..], (&b"12"[..], Option::None)));
}
