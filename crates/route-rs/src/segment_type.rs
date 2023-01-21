use serde::{ Serialize, Deserialize };
// use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum SegmentType<'a> {
    /// Exactly matched
    Static {
        #[serde(borrow)]
        path: &'a str,
    },
    /// Convert single segment value into a param with provided key
    Param {
        #[serde(borrow)]
        key: &'a str,
    },
    /// Convert remaining path value into a param with provided key
    Consume {
        #[serde(borrow)]
        key: &'a str,
    },
    /// Match but discard remaining segments beyond this one
    Wildcard,
}

// impl<'a> SegmentType<'a> {
//     fn from_str(src: &'a str) -> Self
//     // where
//     //     T: Into<Cow<'a, str>>,
//     {
//         // let src = src.into();
//         if src == "*" { return SegmentType::Wildcard; }
//         if src.starts_with(':') { return SegmentType::Param { key: &src[1..] }; }
//         if src.starts_with("*") { return SegmentType::Consume { key: &src[1..] }; }
//         SegmentType::Static { path: &src }
//     }
// }

impl<'a> From<&'a str> for SegmentType<'a> {
    fn from(src: &'a str) -> Self {
        if src == "*" { return SegmentType::Wildcard; }
        if src.starts_with(':') { return SegmentType::Param { key: &src[1..] }; }
        if src.starts_with("*") { return SegmentType::Consume { key: &src[1..] }; }
        SegmentType::Static { path: &src }
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    // Clears codecov misses for derived macros
    fn clone_segment_type() {
        assert_eq!("Wildcard", format!("{:?}", SegmentType::Wildcard));
        assert_eq!(SegmentType::Wildcard, SegmentType::Wildcard.clone());
    }

    #[test]
    fn accept_empty_string_as_static() {
        assert_eq!(SegmentType::Static { path: "" }, "".into());
    }

    #[test]
    // Expect the caller to provide char validation for thier domain
    fn accept_invalid_path_chars() {
        assert_eq!(SegmentType::Static { path: "$%^&#@$" }, "$%^&#@$".into());
    }

    #[test]
    fn parse_static_segment_type() {
        assert_eq!(SegmentType::Static { path: "foo" }, "foo".into());
    }

    #[test]
    fn parse_param_segment_type() {
        assert_eq!(SegmentType::Param { key: "foo" }, ":foo".into());
    }

    #[test]
    // Supports empty string param key
    fn parse_unnamed_param_segment_type() {
        assert_eq!(SegmentType::Param { key: "" }, ":".into());
    }

    #[test]
    fn parse_consume_segment_type() {
        assert_eq!(SegmentType::Consume { key: "foo" }, "*foo".into());
    }

    #[test]
    fn parse_wildcard_segment_type() {
        assert_eq!(SegmentType::Wildcard, "*".into());
    }
}
